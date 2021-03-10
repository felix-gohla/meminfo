use std::convert::TryInto;
use std::error::Error;

use meminfo_server::MeminfoCollector;

use caps::CapSet;
use nix::unistd::{Group, User};
use zbus::{dbus_interface, dbus_proxy, fdo, Connection, ObjectServer};
use zbus_polkit::policykit1::*;

fn main() -> Result<(), Box<dyn Error>> {
    let uid = nix::unistd::getuid();
    if !uid.is_root() {
        return Err("Need root privileges for the meminfo server to run.".into());
    }

    let connection = Connection::new_system()?;
    let proxy = fdo::DBusProxy::new(&connection)?;
    proxy.request_name(
        "de.hpi.felixgohla.meminfo",
        fdo::RequestNameFlags::ReplaceExisting.into(),
    )?;

    let mut object_server = ObjectServer::new(&connection);
    let greeter = MeminfoCollector::new().expect("can initialize MeminfoCollector");
    object_server.at(&"/de/hpi/felixgohla/meminfo".try_into()?, greeter)?;

    drop_caps()?;

    loop {
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("{}", err);
        }
    }

    /* println!("1");
    let connection = Connection::new_system()?;
    println!("2");
    fdo::DBusProxy::new(&connection)?.request_name(
        "de.hpi.felixgohla.meminfo",
        fdo::RequestNameFlags::ReplaceExisting.into(),
    )?;
    println!("3");

    let mut object_server = ObjectServer::new(&connection);
    let collector = MeminfoCollector { count: 42 };
    println!("4");
    object_server.at(&"/de/hpi/felixgohla/meminfo".try_into()?, collector)?;

    drop_caps()?;

    loop {
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("Error handling request: {}", err);
        }
    }*/

    /*let proxy = AuthorityProxy::new(&connection).unwrap();
    let subject = Subject::new_for_owner(std::process::id(), None, None).unwrap();
    let result = proxy.check_authorization(
        &subject,
        "de.hpi.felix-gohla.pkexec.meminfo",
        std::collections::HashMap::new(),
        CheckAuthorizationFlags::AllowUserInteraction.into(),
        "",
    );
    match result {
        Ok(result) => {
            if !result.is_authorized {
                eprintln!("dafuq");
                std::process::exit(-1);
            }
            println!("hello from daemon {}", users::get_effective_uid());
            if users::get_effective_uid() > 0 {
                std::process::Command::new("/bin/pkexec")
                    .arg("/home/fg/meminfo/target/debug/meminfo-daemon")
                    .spawn()
                    .expect("restart");
                std::process::exit(0);
            }
        }
        Err(err) => {
            eprintln!("auth error: {}", err);
        }
    }*/
}

fn drop_caps() -> Result<(), Box<dyn Error>> {
    let nobody = User::from_name("nobody")?.expect("nobody user exists");
    let nogroup = Group::from_name("nogroup")?.expect("nogroup group exists");
    nix::unistd::setgid(nogroup.gid)?;
    nix::unistd::setuid(nobody.uid)?;

    caps::clear(None, CapSet::Effective)?;
    caps::clear(None, CapSet::Inheritable)?;
    caps::clear(None, CapSet::Permitted)?;

    Ok(())
}
