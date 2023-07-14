use seed::{*, prelude::*};

use crate::{Model, Msg};

pub struct NewSaleModel{

}

#[derive(Clone)]
pub enum NewSaleMsg{

}

pub fn update(msg: NewSaleMsg, model: &mut Model, orders: &mut impl Orders<Msg>){
    match msg {
        
    }
}

pub fn view(model: &Model) -> Node<Msg>{
    div![
        "new sale"
    ]
}