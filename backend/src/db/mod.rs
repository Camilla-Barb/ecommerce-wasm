use mongodb::{Client, Database};
use std::sync::Arc;

pub async fn connect() -> Arc<Database> {
    // it reads the environment variable MONGO_URI or uses localhost
    let mongo_uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Errore nella connessione a MongoDB");

    let db = client.database("ecommerce");
    Arc::new(db)
}
