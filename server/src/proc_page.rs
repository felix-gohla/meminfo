use bitflags::bitflags;

bitflags! {
    /// Describes the status of a page frame.
    ///
    /// Also see documentation at: <https://www.kernel.org/doc/Documentation/vm/pagemap.txt>
    pub struct PageFlags: u64 {
        /// page is being locked for exclusive access, eg. by undergoing read/write IO
        const LOCKED        = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
        /// **IO related:** IO error occurred
        const ERROR         = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010;
        /// **LRU related:** page has been referenced since last LRU list enqueue/requeue
        const REFERENCED    = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000100;
        /// **IO related:** page has up-to-date data
        /// ie. for file backed page: (in-memory data revision >= on-disk one)
        const UPTODATE      = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;
        /// **IO related:** page has been written to, hence contains new data
        /// ie. for file backed page: (in-memory data revision >  on-disk one)
        const DIRTY         = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000;
        /// **LRU related:** page is in one of the LRU lists
        const LRU           = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100000;
        /// **LRU related:** page is in the active LRU list
        const ACTIVE        = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000000;
        /// page is managed by the SLAB/SLOB/SLUB/SLQB kernel memory allocator
        /// When compound page is used, SLUB/SLQB will only set this flag on the head
        /// page; SLOB will not flag it at all.
        const SLAB          = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000;
        /// **IO related:** page is being synced to disk
        const WRITEBACK     = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000;
        /// **LRU related:** page will be reclaimed soon after its pageout IO completed
        const RECLAIM       = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00000000;
        /// a free memory block managed by the buddy system allocator
        /// The buddy system organizes free memory in blocks of various orders.
        /// An order N block has 2^N physically contiguous pages, with the BUDDY flag
        /// set for and _only_ for the first page.
        const BUDDY         = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000100_00000000;
        /// **LRU related:** a memory mapped page
        const MMAP          = 0b00000000_00000000_00000000_00000000_00000000_00000000_00001000_00000000;
        /// **LRU related:** a memory mapped page that is not part of a file
        const ANON          = 0b00000000_00000000_00000000_00000000_00000000_00000000_00010000_00000000;
        /// **LRU related:** page is mapped to swap space, ie. has an associated swap entry
        const SWAPCACHE     = 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00000000;
        /// **LRU related:** page is backed by swap/RAM
        const SWAPBACKEND   = 0b00000000_00000000_00000000_00000000_00000000_00000000_01000000_00000000;
        /// A compound page with order N consists of 2^N physically contiguous pages.
        /// A compound page with order 2 takes the form of "HTTT", where H donates its
        /// head page and T donates its tail page(s).  The major consumers of compound
        /// pages are hugeTLB pages (Documentation/vm/hugetlbpage.txt), the SLUB etc.
        /// memory allocators and various device drivers. However in this interface,
        /// only huge/giga pages are made visible to end users.
        const COMPOUND_HEAD = 0b00000000_00000000_00000000_00000000_00000000_00000000_10000000_00000000;
        /// A compound page with order N consists of 2^N physically contiguous pages.
        /// A compound page with order 2 takes the form of "HTTT", where H donates its
        /// head page and T donates its tail page(s).  The major consumers of compound
        /// pages are hugeTLB pages (Documentation/vm/hugetlbpage.txt), the SLUB etc.
        /// memory allocators and various device drivers. However in this interface,
        /// only huge/giga pages are made visible to end users.
        const COMPOUND_TAIL = 0b00000000_00000000_00000000_00000000_00000000_00000001_00000000_00000000;
        /// this is an integral part of a HugeTLB page
        const HUGE          = 0b00000000_00000000_00000000_00000000_00000000_00000010_00000000_00000000;
        /// **LRU related:** page is in the unevictable (non-)LRU list
        /// It is somehow pinned and not a candidate for LRU page reclaims,
        /// eg. ramfs pages, shmctl(SHM_LOCK) and mlock() memory segments
        const UNEVICTABLE   = 0b00000000_00000000_00000000_00000000_00000000_00000100_00000000_00000000;
        /// hardware detected memory corruption on this page: don't touch the data!
        const HWPOISON      = 0b00000000_00000000_00000000_00000000_00000000_00001000_00000000_00000000;
        /// no page frame exists at the requested address
        const NOPAGE        = 0b00000000_00000000_00000000_00000000_00000000_00010000_00000000_00000000;
        /// identical memory pages dynamically shared between one or more processes
        const KSM           = 0b00000000_00000000_00000000_00000000_00000000_00100000_00000000_00000000;
        /// contiguous pages which construct transparent hugepages
        const THP           = 0b00000000_00000000_00000000_00000000_00000000_01000000_00000000_00000000;
        /// tbd
        const OFFLINE       = 0b00000000_00000000_00000000_00000000_00000000_10000000_00000000_00000000;
        /// zero page for pfn_zero or huge_zero page
        const ZERO_PAGE     = 0b00000000_00000000_00000000_00000000_00000001_00000000_00000000_00000000;
        /// page has not been accessed since it was marked idle (see
        /// Documentation/vm/idle_page_tracking.txt). Note that this flag may be
        /// stale in case the page was accessed via a PTE. To make sure the flag
        /// is up-to-date one has to read /sys/kernel/mm/page_idle/bitmap first.
        const IDLE          = 0b00000000_00000000_00000000_00000000_00000010_00000000_00000000_00000000;
        /// contains paging structures
        const PAGETABLE     = 0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000;

   }
}

#[derive(Debug)]
/// Represents the state of a physical page frame.
pub struct PageFrame {
    /// The number of times the frame is used.
    pub reference_count: u64,
    /// The state of the page frame.
    pub flags: PageFlags,
}

#[derive(Default, Debug)]
pub struct LRUPageFrameStats {
    pub total: u64,
    pub active: u64,
    pub inactive: u64,
    pub unevictable: u64,
}

#[derive(Default, Debug)]
pub struct MmapFrameStats {
    pub total: u64,
    pub anon: u64,
    pub file: u64,
}

#[derive(Default, Debug)]
pub struct FreeFramesStats {
    pub total: u64,
    pub noflag: u64,
    /// Frames that are not referenced but still in the LRU lists and
    /// therefore not available in some allocator.
    pub previously_used: u64,
}

#[derive(Default, Debug)]
pub struct HugeFramesStats {
    pub total: u64,
    pub total_fine_granular: u64,
    pub reserved: u64,
    pub reserved_fine_granular: u64,
    pub transparent: u64,
    pub transparent_fine_granular: u64,
}

#[derive(Default, Debug)]
/// Statistics about physical page frames in the system.
pub struct PageFrameStats {
    pub lru_stats: LRUPageFrameStats,

    pub mmaped_stats: MmapFrameStats,

    pub free_stats: FreeFramesStats,

    /// Frames that are shared in KSM.
    pub shared: u64,

    /// Frames with faulty hardware.
    pub poisoned: u64,

    pub buddy: u64,
    pub slab: u64,
    pub zero: u64,
    pub compound: u64,

    pub huge_stats: HugeFramesStats,

    pub pagetable: u64,

    pub frames_in_use: u64,
    pub total_frames: u64,
}
