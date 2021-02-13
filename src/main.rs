mod readinfo;
mod ui;

use readinfo::read_meminfo;
use ui::app;

use gio::prelude::*;
use std::env::args;

fn main() {
    read_meminfo();
    let application = app::create_application();
    application.run(&args().collect::<Vec<_>>());
}
