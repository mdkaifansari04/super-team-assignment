use axum::{Json};
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};
use bs58;

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

pub async fn generate_keypair() -> Json<KeypairResponse> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    Json(KeypairResponse { pubkey, secret })
}
