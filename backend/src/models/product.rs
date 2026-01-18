use async_graphql::{ InputObject, Object};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub _id: ObjectId,

    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

// Metodo helper per GraphQL
#[Object]
impl Product {
    async fn id(&self) -> String {
        self._id.to_hex()
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn price(&self) -> f64 {
        self.price
    }

    async fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    async fn image_url(&self) -> Option<&str> {
        self.image_url.as_deref()
    }
}

#[derive(Debug, InputObject)]
pub struct CreateProductInput {
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateProductInput {
    pub id: String,  
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub price: Option<f64>,
}

