use procfs::Meminfo;

pub fn read_meminfo() -> procfs::Meminfo {
    let info = Meminfo::new();
    match &info {
        Ok(info) => {
            println!("hi {:?}", info.mem_total);
        }
        Err(err) => {
            println!("Err: {:?}", err);
        }
    };
    info.unwrap()
}

pub fn read_pageflags() -> std::io::Result<()> {
    let page_counts = std::fs::read("/proc/kpagecount");
    match page_counts {
        Ok(_) => {
            println!("ok");
            Ok(())
        }
        Err(err) => Err(err),
    }
}
