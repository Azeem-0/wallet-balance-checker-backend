use crate::blockchain::{
    get_citrea_balance, get_erc20_balance, get_sepolia_eth_balance, get_testnet_btc_balance,
};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub address: String,
    pub token: String,
}

pub async fn check_erc20_balance(wallet: web::Json<Wallet>) -> impl Responder {
    let address = wallet.address.clone();
    let token = wallet.token.clone().to_lowercase();
    match get_erc20_balance(address, token).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(err) => {
            dbg!(err);
            return HttpResponse::InternalServerError().body("Failed to fetch balance");
        }
    }
}

pub async fn check_sepolia_eth_balance(wallet: web::Json<Wallet>) -> impl Responder {
    let address = wallet.address.clone();

    match get_sepolia_eth_balance(address).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(err) => {
            dbg!(err);
            return HttpResponse::InternalServerError().body("Failed to fetch balance");
        }
    }
}

pub async fn check_testnet_btc_balance(wallet: web::Json<Wallet>) -> impl Responder {
    let address = wallet.address.clone();

    match get_testnet_btc_balance(address).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(err) => {
            dbg!(err);
            return HttpResponse::InternalServerError().body("Failed to fetch balance");
        }
    }
}

pub async fn check_citrea_balance(wallet: web::Json<Wallet>) -> impl Responder {
    let address = wallet.address.clone();

    match get_citrea_balance(address).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(err) => {
            dbg!(err);
            return HttpResponse::InternalServerError().body("Failed to fetch balance");
        }
    }
}
