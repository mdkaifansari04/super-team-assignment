use axum::{Json, http::StatusCode};
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{Keypair, Signer};
use serde_json::json;
use base58::FromBase58;
use bs58;
use solana_sdk::signature::{Keypair, Signer, Signature};
use solana_sdk::pubkey::Pubkey;
use crate::models::messge::SignMessageRequest;

pub async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Missing required fields" })),
        ));
    }

    let secret_bytes = match payload.secret.from_base58() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "Invalid secret key" })),
            ));
        }
    };

    if secret_bytes.len() != 64 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Secret key must be 64 bytes" })),
        ));
    }

    let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret_bytes[..32]).unwrap();
    let public_key = ed25519_dalek::PublicKey::from_bytes(&secret_bytes[32..]).unwrap();
    let keypair = Keypair { secret: secret_key, public: public_key };

    let signature = keypair.sign(payload.message.as_bytes());

    Ok(Json(json!({
        "success": true,
        "data": {
            "signature": general_purpose::STANDARD.encode(signature.to_bytes()),
            "public_key": bs58::encode(public_key.to_bytes()).into_string(),
            "message": payload.message,
        }
    })))
}

pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> Json<serde_json::Value> {
    // Decode base64 signature
    let signature_bytes = match general_purpose::STANDARD.decode(&payload.signature) {
        Ok(sig) => sig,
        Err(_) => {
            return Json(json!({ "success": false, "error": "Invalid base64 signature" }));
        }
    };

    // Decode base58 public key
    let pubkey_bytes = match bs58::decode(&payload.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(json!({ "success": false, "error": "Invalid base58 pubkey" }));
        }
    };

    // Reconstruct signature and public key
    let signature = Signature::from_bytes(&signature_bytes);
    let public_key = PublicKey::from_bytes(&pubkey_bytes);

    if signature.is_err() || public_key.is_err() {
        return Json(json!({ "success": false, "error": "Invalid key or signature format" }));
    }

    let result = public_key
        .unwrap()
        .verify(payload.message.as_bytes(), &signature.unwrap())
        .is_ok();

    Json(json!({
        "success": true,
        "data": {
            "valid": result,
            "message": payload.message,
            "pubkey": payload.pubkey
        }
    }))
}