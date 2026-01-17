use async_graphql::{Object, Context, Schema, EmptySubscription};
use crate::models::product::{Product, CreateProductInput};
use mongodb::{Database, bson::doc};
use futures::stream::TryStreamExt;
use std::sync::Arc;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(db: Arc<Database>) -> AppSchema {
    Schema::build(QueryRoot,MutationRoot,  EmptySubscription)
        .data(db) // insert the DB in the schema
        .finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // this is a resolver (GraphQL heart)
    async fn products(&self, ctx: &Context<'_>) -> Vec<Product> {
        let db = ctx.data::<Arc<Database>>().unwrap();
        let collection = db.collection::<Product>("products");

        let mut cursor = collection
            .find(doc! {})
            .await
            .expect("Errore nel find");

        let mut products = Vec::new();
        while let Some(prod) = cursor.try_next().await.expect("Errore nel cursor") {
            products.push(prod);
        }

        products
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_product(
        &self,
        ctx: &Context<'_>,
        input: CreateProductInput,
    ) -> Product {
        let db = ctx.data::<Arc<Database>>().unwrap();
        let collection = db.collection::<Product>("products");

        let product = Product {
            _id: mongodb::bson::oid::ObjectId::new(),
            name: input.name,
            price: input.price,
        };

        collection
            .insert_one(product.clone())
            .await
            .expect("Errore insert_one");

        product
    }
}
