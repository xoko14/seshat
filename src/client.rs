use reqwest::Client;
use seshat_schemas::Seller;

pub struct ApiClient{
    pub url: String,
}

impl Default for ApiClient{
    fn default() -> Self {
        Self { url: "http://localhost:8124".to_owned() }
    }
}

impl ApiClient{
    pub fn new(url: &str) -> Self{
        Self { url: url.to_owned() }
    }

    pub async fn get_sellers(&self) -> Vec<Seller>{
        let res = Client::new()
            .get(format!("{}/{}", self.url, "sellers"))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        res
    }
}