use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Seller {
    pub id: i64,
    pub name: String,
    pub stand_number: i64,
    pub discount: f64,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SellerCreate {
    pub name: String,
    pub stand_number: i64,
    pub discount: f64,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub ean: String,
    pub pvp: f64,
    pub language: String,
    pub stock: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleCreate {
    pub title: String,
    pub ean: String,
    pub pvp: f64,
    pub language: String,
    pub stock: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaleLine {
    pub article_ean: String,
    pub total_line: f64,
    pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sale {
    pub seller_id: i64,
    pub total: f64,
    pub detail: Vec<SaleLine>,
}
