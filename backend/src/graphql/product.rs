// product.rs
use async_graphql::SimpleObject;

#[derive(SimpleObject, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: String,
}
