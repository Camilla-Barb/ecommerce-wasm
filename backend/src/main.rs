mod graphql;
mod db;
mod models;

use axum::{
    routing::{get},
    Router, extract::State, response::Html,
};
use graphql::{create_schema, AppSchema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use std::{sync::Arc, net::SocketAddr};
use db::connect;
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() {
    // 1️⃣ Connection to DB
    let db = connect().await;

    // 2️⃣ Creation schema GraphQL + DB insertion inside schema
    let schema = Arc::new(create_schema(db.clone()));

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    // 3️⃣ Router with only one state: the schema
    let app = Router::new() .route("/graphql", get(graphiql).post(graphql_handler)) .with_state(schema.clone()).layer(cors);

    // 4️⃣ Server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server in ascolto su http://{}", addr);

    // 5️⃣ Listener TCP + server start
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Impossibile aprire la porta");

    axum::serve(listener, app)
        .await
        .expect("Errore durante l'esecuzione del server");
}

// --- Handler GraphQL ---
async fn graphql_handler( State(schema): State<Arc<AppSchema>>, req: GraphQLRequest, ) -> GraphQLResponse { schema.execute(req.into_inner()).await.into() }

// --- Handler GraphiQL ---
async fn graphiql() -> Html<String> {
    Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/graphql")
            .finish()
    )
}
