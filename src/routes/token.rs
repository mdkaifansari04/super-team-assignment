use axum::Json;
use serde_json::json;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::str::FromStr;
use crate::models::token::{TokenRequest, MintTokenRequest};
use base64::{engine::general_purpose, Engine as _};


pub async fn create_token(Json(payload): Json<TokenRequest>) -> Json<serde_json::Value>
 {
    let mint_pubkey = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid mint public key"
            }));
        }
    };

    let mint_authority = match Pubkey::from_str(&payload.mint_authority) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid mint authority public key"
            }));
        }
    };

    let accounts = vec![
        AccountMeta::new(mint_pubkey, false),
        AccountMeta::new_readonly(mint_authority, true),
    ];

    let ix_data = vec![payload.decimals, 0, 0, 0, 0, 0, 0, 0]; // dummy instruction data

    let instruction = Instruction {
        program_id: spl_token::id(),
        accounts: accounts.clone(),
        data: ix_data.clone(),
    };


    Json(json!({
        "success": true,
        "data": {
            "program_id": instruction.program_id.to_string(),
            "accounts": accounts.iter().map(|a| {
                json!({
                    "pubkey": a.pubkey.to_string(),
                    "is_signer": a.is_signer,
                    "is_writable": a.is_writable
                })
            }).collect::<Vec<_>>(),
            "instruction_data": general_purpose::STANDARD.encode(&ix_data),
        }
    }))
}

pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> Json<serde_json::Value> {
    let mint = Pubkey::from_str(&payload.mint).unwrap();
    let dest = Pubkey::from_str(&payload.destination).unwrap();
    let authority = Pubkey::from_str(&payload.authority).unwrap();

    let accounts = vec![
        AccountMeta::new(mint, false),
        AccountMeta::new(dest, true),
        AccountMeta::new_readonly(authority, true),
    ];

    let ix_data = payload.amount.to_le_bytes().to_vec(); // dummy data

    let instruction = Instruction {
        program_id: spl_token::id(),
        accounts: accounts.clone(),
        data: ix_data.clone(),
    };

    Json(json!({
        "success": true,
        "data": {
            "program_id": instruction.program_id.to_string(),
            "accounts": accounts.iter().map(|a| {
                json!({
                    "pubkey": a.pubkey.to_string(),
                    "is_signer": a.is_signer,
                    "is_writable": a.is_writable
                })
            }).collect::<Vec<_>>(),
            "instruction_data": general_purpose::STANDARD.encode(ix_data),
        }
    }))
}