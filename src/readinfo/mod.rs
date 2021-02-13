use procfs::Meminfo;

pub fn read_meminfo() {
    let info = Meminfo::new();
    match info {
        Ok(info) => {
            println!("hi {:?}", info.mem_total);
        }
        Err(err) => {
            println!("Err: {:?}", err);
        }
    }
}
