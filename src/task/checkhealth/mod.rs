pub mod autofixable;
pub mod tips;

// use crate::task;

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
        if let Some((msg, tip)) = autofixable::vpn::is_disconnected() {
            cond_connect(ask((msg, tip)));
        }
        if let Some((msg, tip)) = autofixable::aegis_apps_is_empty() {
            cond_up_aegis_apps(ask((msg, tip)));
        }
    }
}
