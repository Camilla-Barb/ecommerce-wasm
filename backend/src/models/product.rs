use async_graphql::{SimpleObject, InputObject};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct Product {
    #[graphql(skip)]
    pub _id: ObjectId,

    pub name: String,
    pub price: f64,
}

#[derive(Debug, InputObject)]
pub struct CreateProductInput {
    pub name: String,
    pub price: f64,
}
