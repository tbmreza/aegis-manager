pub mod autofixable;
pub mod tips;

use crate::task;

fn ask() -> bool {
    // true
    false
}

fn cond_connect(cond: bool) {
    if cond {
        // autofixable::vpn::connect();
        println!("autofixable::vpn::connect()...");
    }
}

fn cond_up_aegis_apps(cond: bool) {
    if cond {
        // task::up_aegis_apps(1);
        println!("task::up_aegis_apps(1)...");
    }
}

pub fn run(yes_to_all: bool) {
    tips::check();

    if yes_to_all {
        // autofixable::vpn::connect();
        println!("autofixable::vpn::connect()...");

        // task::up_aegis_apps(1);
        println!("task::up_aegis_apps(1)...");
    } else {
        if autofixable::vpn::is_disconnected() {
            cond_connect(ask());
        }
        if task::docker_ps_format_id_names().is_empty() {
            cond_up_aegis_apps(ask());
        }
    }
}
