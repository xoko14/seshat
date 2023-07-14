use seed::{prelude::*, virtual_dom::Node, *};

use crate::{Model, Msg};

use self::{newsale::NewSaleMsg, sellers::SellersMsg};

pub mod newsale;
pub mod sellers;

#[derive(Clone)]
pub enum Section {
    None,
    NewSale,
    Info,
    Sellers,
}

#[derive(Clone)]
pub enum SectionMsg {
    NewSaleEvent(NewSaleMsg),
    SellersEvent(SellersMsg),
}

pub fn update(msg: SectionMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        SectionMsg::NewSaleEvent(msg) => newsale::update(msg, model, orders),
        SectionMsg::SellersEvent(msg) => sellers::update(msg, model, orders),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["section"],
        match model.section {
            Section::None => div![],
            Section::NewSale => newsale::view(model),
            Section::Info => div!["info"],
            Section::Sellers => sellers::view(model),
        }
    ]
}
