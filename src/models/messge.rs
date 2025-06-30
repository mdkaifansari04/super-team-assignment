use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String, // base64
    pub pubkey: String,    // base58
}