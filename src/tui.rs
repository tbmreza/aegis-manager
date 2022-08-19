// Real-time selectable list.
// aegis stop -i

use crate::common::*;
use crate::task;
use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;
use tokio::sync::{mpsc, watch};
use tokio::time::{self, Duration};

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

#[derive(Debug)]
pub enum Sig {
    Quit,
    Nop,
}

#[tokio::main]
pub async fn index() {
    flexi_logger::Logger::try_with_str("info, my::critical::module=trace")
        .unwrap()
        .log_to_file(flexi_logger::FileSpec::default().directory("log_files"))
        .write_mode(flexi_logger::WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    // tokio transmitter {
    let mut apps = running_apps();

    let (tx, mut rx) = mpsc::channel(1);
    let (apps_tx, apps_rx) = watch::channel(apps.clone());

    tokio::spawn(async move {
        loop {
            time::sleep(Duration::from_secs(2)).await;

            let new_apps = running_apps();
            if new_apps != apps {
                tx.send(Sig::Quit).await.unwrap();
                apps_tx.send(new_apps.clone()).unwrap();
                apps = new_apps;
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
    siv.run(); // TODO blocks the thread

    while let Some(s) = rx.recv().await {
        match s {
            Sig::Quit => {
                // ...
                log::info!("rx contains: {:?}", s);
            }
            _ => {}
        }
    }
}

fn send_docker_stop(siv: &mut Cursive, app_id: &str) {
    task::docker_stop_by_id(app_id);

    let text = format!("Stopped docker container `{}`.", app_id);
    siv.add_layer(Dialog::around(TextView::new(text)).button("OK", |s| {
        s.pop_layer();
    }));
}
