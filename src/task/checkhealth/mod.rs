pub mod autofixable;
pub mod tips;

// use crate::task;

pub fn run(yes_to_all: bool) {
    // task::up_aegis_apps(1);
    autofixable::vpn::run(yes_to_all);

    // tips::run();
}
