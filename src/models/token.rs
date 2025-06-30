use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub mint: String,
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub decimals: u8,
    }


#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}