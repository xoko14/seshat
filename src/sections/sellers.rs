use seed::{*, prelude::*};

use crate::{Model, Msg};

pub struct SellersModel{

}
#[derive(Clone)]
pub enum SellersMsg{

}

pub fn update(msg: SellersMsg, model: &mut Model, orders: &mut impl Orders<Msg>){
    match msg {
        
    }
}

pub fn view(model: &Model) -> Node<Msg>{
    div![
        "sellers"
    ]
}