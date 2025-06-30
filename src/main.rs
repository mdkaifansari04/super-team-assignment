use axum::{Router, routing::{ post}};
use std::net::SocketAddr;

mod routes;
mod models;
use routes::{
    keypair::generate_keypair,
    token::{create_token, mint_token},
};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on: http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
