use seed::{*, prelude::* };

use crate::Msg;

pub fn button_icon(icon: &str, click_handler: impl FnOnce(web_sys::Event) -> Msg + 'static + Clone) -> Node<Msg>{
    button![
        C!["button"],
        span![
            C!["material-symbols-rounded"],
            icon
        ],
        ev(Ev::Click, click_handler)
    ]
}