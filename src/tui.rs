// Real-time selectable list.
// aegis stop -i

use crate::common::*;
use crate::task;
use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;

fn running_apps() -> RunningApps {
    let m = task::docker_ps_format_id_names();
    RunningApps(m)
}

fn select_aegis_service() -> OnEventView<SelectView> {
    let mut view = SelectView::new().h_align(HAlign::Center).autojump();

    for (app_name, app_id) in running_apps().0 {
        view.add_item(app_name, app_id);
    }
    view.set_on_submit(send_docker_stop);

    OnEventView::new(view)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        })
}

use tokio::sync::watch;

#[derive(Debug)]
pub enum Sig {
    Quit,
}

// tokio watch example {
use std::io;
use tokio::time::{self, Duration, Instant};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Config {
    timeout: Duration,
}

impl Config {
    async fn load_from_file() -> io::Result<Config> {
        // file loading and deserialization logic here
        Ok(Config::default())
    }
}

async fn my_async_operation() {
    // Do something here
}
// }

#[tokio::main]
pub async fn index() {
    // tokio watch example {
    // Load initial configuration value
    let mut config = Config::load_from_file().await.unwrap();

    // Create the watch channel, initialized with the loaded configuration
    let (tx, rx) = watch::channel(config.clone());

    // Spawn a task to monitor the file.
    tokio::spawn(async move {
        loop {
            // Wait 10 seconds between checks
            time::sleep(Duration::from_secs(10)).await;

            // Load the configuration file
            let new_config = Config::load_from_file().await.unwrap();

            // If the configuration changed, send the new config value
            // on the watch channel.
            if new_config != config {
                tx.send(new_config.clone()).unwrap();
                config = new_config;
            }
        }
    });
    // }

    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(select_aegis_service().scrollable().fixed_size((20, 10)))
            .title("Running aegis apps:"),
    );

    siv.add_global_callback('q', Cursive::quit);
    // siv.cb_sink().send(Box::new(|s| s.quit())).unwrap();  // runs for a blip then quit
    siv.run();
    // tokio watch example {
    // Sig::Quit => siv.cb_sink().send(Box::new(|s| s.quit())).unwrap()
    let mut handles = vec![];

    // Spawn tasks that runs the async operation for at most `timeout`. If
    // the timeout elapses, restart the operation.
    //
    // The task simultaneously watches the `Config` for changes. When the
    // timeout duration changes, the timeout is updated without restarting
    // the in-flight operation.
    for _ in 0..5 {
        // Clone a config watch handle for use in this task
        let mut rx = rx.clone();

        let handle = tokio::spawn(async move {
            // Start the initial operation and pin the future to the stack.
            // Pinning to the stack is required to resume the operation
            // across multiple calls to `select!`
            let op = my_async_operation();
            tokio::pin!(op);

            // Get the initial config value
            let mut conf = rx.borrow().clone();

            let mut op_start = Instant::now();
            let sleep = time::sleep_until(op_start + conf.timeout);
            tokio::pin!(sleep);

            loop {
                tokio::select! {
                    _ = &mut sleep => {
                        // The operation elapsed. Restart it
                        op.set(my_async_operation());

                        // Track the new start time
                        op_start = Instant::now();

                        // Restart the timeout
                        sleep.set(time::sleep_until(op_start + conf.timeout));
                    }
                    _ = rx.changed() => {
                        conf = rx.borrow().clone();

                        // The configuration has been updated. Update the
                        // `sleep` using the new `timeout` value.
                        sleep.as_mut().reset(op_start + conf.timeout);
                    }
                    _ = &mut op => {
                        // The operation completed!
                        return
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles.drain(..) {
        handle.await.unwrap();
    }
    // }
}

fn send_docker_stop(siv: &mut Cursive, app_id: &str) {
    task::docker_stop_by_id(app_id);

    let text = format!("Stopped docker container `{}`.", app_id);
    siv.add_layer(Dialog::around(TextView::new(text)).button("OK", |s| {
        s.pop_layer();
    }));
}
