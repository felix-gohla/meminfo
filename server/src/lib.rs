mod proc_page;

use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::iter::Iterator;

use zbus::dbus_interface;

use proc_page::{PageFlags, PageFrame, PageFrameStats};

pub struct MeminfoCollector {
    page_count_fd: File,
    page_flags_fd: File,
    page_frames: Vec<Option<PageFrame>>,
}

impl MeminfoCollector {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let page_count_fd = File::open("/proc/kpagecount")?;
        let page_flags_fd = File::open("/proc/kpageflags")?;

        Ok(Self {
            page_count_fd,
            page_flags_fd,
            page_frames: Vec::new(),
        })
    }

    fn read_page_use_counts(&mut self) -> Vec<u64> {
        let u64_size = std::mem::size_of::<u64>();
        self.page_count_fd.seek(SeekFrom::Start(0)).unwrap();
        let mut page_count_content = Vec::new();
        let bytes_read = self
            .page_count_fd
            .read_to_end(&mut page_count_content)
            .expect("read /proc/kpagecount");
        assert_eq!(bytes_read % u64_size, 0);

        let mut counts = Vec::with_capacity(bytes_read / u64_size);
        counts.extend_from_slice(
            safe_transmute::transmute_many_pedantic::<u64>(&page_count_content)
                .expect("transmute u64"),
        );
        counts
    }

    fn read_page_flags(&mut self) -> Vec<PageFlags> {
        let u64_size = std::mem::size_of::<u64>();

        self.page_flags_fd.seek(SeekFrom::Start(0)).unwrap();
        let mut page_flags_content = Vec::new();
        let bytes_read = self
            .page_flags_fd
            .read_to_end(&mut page_flags_content)
            .expect("read /proc/kpageflags");
        assert_eq!(bytes_read % u64_size, 0);

        let mut flags = Vec::with_capacity(bytes_read / u64_size);
        flags.extend(
            safe_transmute::transmute_many_pedantic::<u64>(&page_flags_content)
                .expect("transmute u64")
                .iter()
                .map(|flag_bits| PageFlags::from_bits_truncate(*flag_bits)),
        );
        flags
    }
}

#[dbus_interface(name = "de.hpi.felixgohla.meminfo.meminfo_collector")]
impl MeminfoCollector {
    fn refresh_physical(&mut self) -> String {
        let counts = self.read_page_use_counts();
        let flags = self.read_page_flags().into_iter();
        let mut page_frames = Vec::with_capacity(counts.len());
        let mut stats = PageFrameStats::default();
        page_frames.extend(
            flags
                .zip(counts.into_iter())
                .map(|(flags, reference_count)| {
                    if !flags.contains(PageFlags::NOPAGE) {
                        // Stats collection.
                        if flags.contains(PageFlags::LRU) {
                            stats.lru_stats.total += 1;
                            if flags.contains(PageFlags::ACTIVE) {
                                assert_eq!(flags.contains(PageFlags::UNEVICTABLE), false, "Active pages should not be unevictable.");
                                stats.lru_stats.active += 1;
                            } else if flags.contains(PageFlags::UNEVICTABLE) {
                                stats.lru_stats.unevictable += 1;
                            } else {
                                stats.lru_stats.inactive += 1;
                            }
                        }
                        if flags.contains(PageFlags::HWPOISON) {
                            stats.poisoned += 1;
                        } else if flags.contains(PageFlags::KSM) {
                            if reference_count < 2 {
                                unreachable!();
                            }
                            stats.shared += 1;
                        } else if flags.contains(PageFlags::BUDDY) {
                            stats.buddy+= 1;
                        } else if flags.contains(PageFlags::SLAB) {
                            stats.slab+= 1;
                        } else if flags.contains(PageFlags::PAGETABLE) {
                            stats.pagetable += 1;
                        } else if flags.contains(PageFlags::MMAP) {
                            stats.mmaped_stats.total += 1;
                            if flags.contains(PageFlags::ANON) {
                                stats.mmaped_stats.anon += 1;
                            } else {
                                stats.mmaped_stats.file += 1;
                            }
                        } else if flags.contains(PageFlags::ZERO_PAGE) {
                            assert_eq!(flags & PageFlags::all(), flags);
                            stats.zero += 1;
                        } else if flags.is_empty() {
                            stats.free_stats.total += 1;
                            stats.free_stats.noflag += 1;
                        } else {
                            assert_eq!(
                                reference_count, 0,
                                "expected unused page (rc=0), but got {} with flags {:?}",
                                reference_count, flags
                            );
                            stats.free_stats.total += 1;
                            stats.free_stats.previously_used += 1;
                        }
                        if flags.intersects(PageFlags::COMPOUND_HEAD | PageFlags::COMPOUND_TAIL) {
                            stats.compound += 1; // This is wrong.
                        }
                        if flags.contains(PageFlags::HUGE) {
                            if flags.intersects(PageFlags::COMPOUND_HEAD) {
                                stats.huge_stats.reserved += 1;
                                stats.huge_stats.total += 1;
                            }
                            stats.huge_stats.reserved_fine_granular += 1;
                            stats.huge_stats.total_fine_granular += 1;
                        }
                        if flags.contains(PageFlags::THP) {
                            if flags.intersects(PageFlags::COMPOUND_HEAD) {
                                stats.huge_stats.transparent += 1;
                                stats.huge_stats.total += 1;
                            }
                            stats.huge_stats.transparent_fine_granular += 1;
                            stats.huge_stats.total_fine_granular += 1;
                        }
                        if reference_count > 0 {
                            stats.frames_in_use += 1;
                        }
                        stats.total_frames += 1;
                        Some(PageFrame {
                            reference_count,
                            flags,
                        })
                    } else {
                        None
                    }
                }),
        );
        assert_eq!(
            stats.mmaped_stats.total
                + stats.slab
                + stats.buddy
                + stats.zero
                + stats.poisoned
                + stats.pagetable
                + stats.shared
                + stats.free_stats.total,
            stats.total_frames
        );
        self.page_frames = page_frames;
        let page_size = nix::unistd::sysconf(nix::unistd::SysconfVar::PAGE_SIZE)
            .expect("get page size sysconf")
            .unwrap() as u64;
        println!("stats: {:?}", stats);
        for (pfn, pf) in self.page_frames.iter().enumerate().filter(|(_, pf)| {
            pf.is_some()
                && pf.as_ref().unwrap().flags.intersects(PageFlags::HUGE | PageFlags::THP)
                /*&& !pf
                    .as_ref()
                    .unwrap()
                    .flags
                    .intersects(PageFlags::MMAP | PageFlags:: BUDDY | PageFlags::KSM | PageFlags::SLAB | PageFlags::HWPOISON | PageFlags::PAGETABLE)*/
                    && !pf.as_ref().unwrap().flags.is_empty()
        }) {
            println!("0x{:X}: {:?}", pfn as u64 * page_size, pf);
        }
        format!(
            "PFs: {} {}",
            self.page_frames.len(),
            self.page_frames
                .iter()
                .filter(|pf| pf.is_some()
                    && pf
                        .as_ref()
                        .unwrap()
                        .flags
                        .contains(PageFlags::DIRTY | PageFlags::ANON))
                .count()
        )
    }
}
