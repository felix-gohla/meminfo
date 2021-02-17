mod model;
mod readinfo;
mod ui;

use model::Overview;
use readinfo::read_meminfo;
use ui::app;

use std::env::args;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    read_meminfo();
    let overview: Arc<Overview> = Arc::new(Default::default());
    let application = app::App::new(overview.clone());
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(5000));
        println!("update value");
        overview
            .max_ram
            .store(123, std::sync::atomic::Ordering::Relaxed);
    });
    application.run(&args().collect::<Vec<_>>());
}
