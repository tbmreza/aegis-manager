// Two panels: live docker ps & selectable list.
// aegis stop -i

use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;
use std::collections::HashMap;

fn running_apps() -> HashMap<String, String> {
    use crate::task::docker_ps_format_id_names;

    docker_ps_format_id_names()
}

fn select_aegis_service() -> OnEventView<SelectView> {
    let mut view = SelectView::new().h_align(HAlign::Center).autojump();

    for (app_name, app_id) in running_apps() {
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

/// Main function.
pub fn tui() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(select_aegis_service().scrollable().fixed_size((20, 10)))
            .title("Running aegis apps:"),
    );

    siv.run();
}

fn send_docker_stop(siv: &mut Cursive, app_id: &str) {
    use crate::task::docker_stop_by_id;
    docker_stop_by_id(app_id);

    let text = format!("Stopped docker container `{}`.", app_id);
    siv.add_layer(Dialog::around(TextView::new(text)).button("OK", |s| {
        s.pop_layer();
    }));
}
