use async_graphql::{Object, Context, Schema};
use crate::models::product::{Product, CreateProductInput, UpdateProductInput};
use mongodb::{Database, bson::doc};
use futures::stream::TryStreamExt;
use std::sync::Arc;

pub type AppSchema =
    async_graphql::Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;


pub fn create_schema(db: Arc<Database>) -> AppSchema {
    Schema::build(QueryRoot,MutationRoot,async_graphql::EmptySubscription)
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
    // create product
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
            description: input.description,
            image_url: input.image_url,
        };

        collection.insert_one(product.clone()).await.unwrap();
        product
    }

    //update product
    async fn update_product(
        &self,
        ctx: &Context<'_>,
        input: UpdateProductInput,
    ) -> bool {
        let db = ctx.data::<Arc<Database>>().unwrap();
        let collection = db.collection::<Product>("products");

        let mut update = mongodb::bson::doc! {};

        if let Some(name) = &input.name {
            update.insert("name", name);
        }

        if let Some(price) = input.price {
            update.insert("price", price);
        }
        if let Some(desc) = input.description {
            update.insert("description", desc);
        }
        if let Some(img) = input.image_url {
            update.insert("image_url", img);
        }

        let object_id = mongodb::bson::oid::ObjectId::parse_str(&input.id)
    .expect("Invalid ObjectId format");

        let result = collection
            .update_one(
                mongodb::bson::doc! { "_id": object_id },
                mongodb::bson::doc! { "$set": update },
            )
            .await
            .unwrap();

        result.matched_count == 1
    }
}


