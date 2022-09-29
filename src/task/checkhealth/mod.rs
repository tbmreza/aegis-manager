pub mod autofixable;
pub mod tips;

use crate::task;

fn ask() -> bool {
    // true
    false
}

fn cond_connect(cond: bool) {
    if cond {
        autofixable::vpn::connect();
    }
}

fn cond_up_aegis_apps(cond: bool) {
    if cond {
        task::up_aegis_apps(1);
    }
}

mod picker {
    pub fn apps() -> Vec<String> {
        vec![String::from("cuirass")]
    }
}

pub fn run(yes_to_all: bool) {
    tips::check();

    if yes_to_all {
        autofixable::vpn::connect();
        task::up_aegis_apps(1);
    } else {
        if autofixable::vpn::is_disconnected() {
            cond_connect(ask());
        }
        if picker::apps().is_empty() {
            cond_up_aegis_apps(ask());
        }
    }
}
