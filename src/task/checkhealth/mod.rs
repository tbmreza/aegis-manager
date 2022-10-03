pub mod autofixable;
pub mod tips;

use crate::task;

fn ask((msg, tip): (String, String)) -> bool {
    use inquire::Confirm;

    let ans = Confirm::new(&msg)
        .with_default(true)
        .with_help_message(&tip)
        .prompt();

    match ans {
        Ok(true) => true,
        _ => false,
    }
}

fn cond_connect(cond: bool) {
    if cond {
        if cfg!(feature = "dummy") {
            println!("autofixable::vpn::connect()...");
        } else {
            autofixable::vpn::connect();
        }
    }
}

fn cond_up_aegis_apps(cond: bool) {
    if cond {
        if cfg!(feature = "dummy") {
            println!("task::up_aegis_apps(1)...");
        } else {
            task::up_aegis_apps(1);
        }
    }
}

pub fn run(yes_to_all: bool) {
    tips::check();

    if yes_to_all {
        if cfg!(feature = "dummy") {
            println!("autofixable::vpn::connect()...");
        } else {
            autofixable::vpn::connect();
        }

        if cfg!(feature = "dummy") {
            println!("task::up_aegis_apps(1)...");
        } else {
            task::up_aegis_apps(1);
        }
    } else {
        if let Some((msg, tip)) = autofixable::vpn::is_disconnected() {
            cond_connect(ask((msg, tip)));
        }
        if let Some((msg, tip)) = autofixable::aegis_apps_is_empty() {
            cond_up_aegis_apps(ask((msg, tip)));
        }
    }
}
