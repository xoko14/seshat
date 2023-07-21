use std::panic;

use sections::{SectionMsg, Section, sellers::SellersModel};
use seed::{prelude::*, *};

mod glue;
mod utils;
mod client;
mod lang;
mod sections;
mod components;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::start("app", init, update, view);
}

pub struct Model {
    server_loaded: bool,
    section: Section,
    sellers_model: SellersModel,
}

#[derive(Clone)]
pub enum Msg {
    ServerLoaded(String),
    ChangeSection(Section),
    SectionEvent(SectionMsg)
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.skip().perform_cmd({
        async {
            glue::run_server().await;
            let text = reqwest::get("http://localhost:8124/test").await.unwrap().text().await.unwrap();
            Msg::ServerLoaded(text)
        }
    });

    Model {
        server_loaded: false,
        section: Section::None,
        sellers_model: Default::default(),
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ServerLoaded(text) => {
            model.server_loaded = true;
        },
        Msg::ChangeSection(section) => model.section = section,
        Msg::SectionEvent(msg) => sections::update(msg, model, orders),
    }
}

fn view(model: &Model) -> Node<Msg> {
    if !model.server_loaded {
        return span![];
    }

    div![
        C!["flex", "transition-all"],
        view_sidebar(),
        sections::view(model)
    ]
}

fn view_sidebar() -> Node<Msg>{
    div![
        C!["sidebar"],
        view_sidebar_icon("point_of_sale", &t!("section-newsale"), |_| Msg::ChangeSection(Section::NewSale)),
        view_sidebar_icon("storefront", &t!("section-sellers"), |_| Msg::ChangeSection(Section::Sellers)),
        view_sidebar_icon("info", &t!("section-about"), |_| Msg::ChangeSection(Section::Info))
    ]
}

fn view_sidebar_icon(icon: &str, tooltip: &str, click_handler: impl FnOnce(web_sys::Event) -> Msg + 'static + Clone) -> Node<Msg>{
    button![
        C!["sidebar-icon", "group"],
        span![
            C!["material-symbols-rounded", "text-4xl"],
            icon
        ],
        span![
            C!["sidebar-tooltip", "group-hover:scale-100"],
            tooltip
        ],
        ev(Ev::Click, click_handler)
    ]
}
