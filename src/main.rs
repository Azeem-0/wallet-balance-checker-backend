use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use handlers::{
    check_citrea_balance, check_erc20_balance, check_sepolia_eth_balance, check_testnet_btc_balance,
};

mod asset_model;
mod blockchain;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/check-erc20", web::post().to(check_erc20_balance))
            .route(
                "/check-sepolia-eth",
                web::post().to(check_sepolia_eth_balance),
            )
            .route("/check-citrea", web::post().to(check_citrea_balance))
            .route(
                "/check-testnet-btc",
                web::post().to(check_testnet_btc_balance),
            )
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
