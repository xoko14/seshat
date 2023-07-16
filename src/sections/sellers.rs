use seed::{prelude::*, *};
use seshat_schemas::Seller;

use crate::{sections::SectionMsg, t, Model, Msg, client::ApiClient};

pub struct SellersModel {
    sellers: Option<Vec<Seller>>,
}

impl Default for SellersModel {
    fn default() -> Self {
        Self {
            sellers: Default::default(),
        }
    }
}

#[derive(Clone)]
pub enum SellersMsg {
    LoadData,
    DataLoaded(Vec<Seller>),
}

pub fn update(msg: SellersMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        SellersMsg::LoadData => {
            orders.perform_cmd(async {
                let sellers = ApiClient::default().get_sellers().await;
                Msg::SectionEvent(SectionMsg::SellersEvent(SellersMsg::DataLoaded(sellers)))
            });
        },
        SellersMsg::DataLoaded(s) => {
            model.sellers_model.sellers = Some(s);
        },
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        "sellers",
        button!(
            "temp load",
            ev(Ev::Click, |_| Msg::SectionEvent(SectionMsg::SellersEvent(
                SellersMsg::LoadData
            )))
        ),
        match &model.sellers_model.sellers {
            Some(s) => view_table(s),
            None => div!["no content"],
        }
    ]
}

pub fn view_table(sellers: &Vec<Seller>) -> Node<Msg> {
    table![
        th![td![t!("name")], td![t!("stand-number")], td![t!("code")]],
        sellers
            .iter()
            .map(|s| { tr![td![&s.name], td![&s.stand_number], td![&s.code]] })
    ]
}
