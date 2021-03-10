mod model;
mod readinfo;
mod ui;

use model::Overview;
use ui::app;
use ui::dispatch::DispatchLoop;

use std::convert::TryInto;
use std::env::args;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    run_server().unwrap();
    let overview: Arc<Overview> = Arc::new(Default::default());

    let dispatch_loop = DispatchLoop::new();
    let sender = dispatch_loop.make_dispatcher();

    let application = app::App::new(overview.clone(), sender.clone());

    {
        let sender = sender.clone();
        thread::spawn(move || {
            use std::collections::HashMap;
            use zbus::Connection;
            use zbus_polkit::policykit1::*;

            let connection = Connection::new_system().unwrap();
            let proxy = AuthorityProxy::new(&connection).unwrap();
            let subject = Subject::new_for_owner(std::process::id(), None, None).unwrap();
            let result = proxy.check_authorization(
                &subject,
                "de.hpi.felix-gohla.pkexec.meminfo",
                HashMap::new(),
                CheckAuthorizationFlags::AllowUserInteraction.into(),
                "",
            );
            if let Err(err) = result {
                eprintln!("Error during authentication: {}", err);
                std::process::exit(-1);
            }
            let res = result.unwrap();

            if !res.is_authorized {
                eprintln!("Was not authorized.");
                std::process::exit(-1);
            }

            println!("metadata for {:?}", users::get_effective_uid());

            loop {
                match readinfo::read_pageflags() {
                    Ok(_) => {
                        println!("Can read, have permissions.");
                    }
                    Err(err) => {
                        sender
                            .unbounded_send(ui::AppAction::ShowNoRootDialog)
                            .unwrap();
                        eprintln!("Error {:?}", err);
                    }
                }
                thread::sleep(Duration::from_millis(5000));
                let info = readinfo::read_meminfo();
                overview
                    .ram_total
                    .store(info.mem_total.try_into().unwrap(), Ordering::Relaxed);
                overview
                    .ram_free
                    .store(info.mem_free.try_into().unwrap(), Ordering::Relaxed);
                sender.unbounded_send(ui::AppAction::MeminfoUpdate).unwrap();
            }
        });
    }
    application.run(args().collect::<Vec<_>>(), dispatch_loop);
}

fn run_server() -> Result<usize, String> {
    Err("not implemented.".to_string())
}
