use client::ApiClient;
use seed::{prelude::*, *};
use seshat_schemas::Seller;

mod glue;
mod utils;
mod client;

fn main() {
    App::start("app", init, update, view);
}

struct Model {
    server_loaded: bool,
    text: String,
    sellers: Vec<Seller>
}

#[derive(Clone)]
enum Msg {
    ServerLoaded(String),
    DataLoading,
    DataLoaded(Vec<Seller>)
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
        text: "Hello world".to_owned(),
        sellers: Vec::new(),
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ServerLoaded(text) => {
            model.server_loaded = true;
            model.text = text;
        },
        Msg::DataLoading => {
            orders.skip().perform_cmd(async{
                let client = ApiClient::default();
                Msg::DataLoaded(client.get_sellers().await)
            });
        },
        Msg::DataLoaded(sellers) => model.sellers = sellers,
    }
}

fn view(model: &Model) -> Node<Msg> {
    if !model.server_loaded {
        return span!["loading..."];
    }

    div![
        &model.text,
        button![
            "load data",
            ev(Ev::Click, |_| Msg::DataLoading)
        ],
        ul![
            C!["p-10"],
            model.sellers.iter().map(|s| li![&s.name])
        ]
    ]
}
