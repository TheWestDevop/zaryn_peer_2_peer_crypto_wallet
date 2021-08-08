extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate rustc_serialize;


mod actors;
mod middleware;
mod models;
mod routes;
mod schema;
mod utils;
mod controllers;

use actix_web::{App, HttpServer,  middleware::{Logger}, web};
use models::state::AppState;
use routes::wallet:: { get_wallets, create_wallet, update_wallet, get_wallet_info, delete_wallet};
use routes::transactions:: { get_transactions, get_wallet_transactions, get_transaction_info  };
use routes::stake::*;
use routes::routes_state::*;
use tracing::instrument;
use utils::{config::boot};

#[actix_web::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    let boot = boot().await;
    let boot_copy = boot.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a security token ==>%{authorization}i"))
            .service(health)
            .service(
                web::scope("/v1/wallet")
                    .service(get_wallets)
                    .service(get_wallet_info)
                    .service(create_wallet)
                    .service(update_wallet)
                    .service(delete_wallet),
            ).service(
                web::scope("/v1/transaction")
                    .service(get_transactions)
                    .service(get_wallet_transactions)
                    .service(get_transaction_info),
            ).service(
                web::scope("/v1/stake")
                    .service(get_stakes)
                    .service(get_stake_info)
                    .service(create_stake)
                    .service(transfer_my_stake),
            )
            .default_service(
                web::route().to(route_not_found), //
            )
            .data(AppState::new(boot.db.clone()))
    })
    .workers(boot_copy.workers)
    .bind(format!("{}:{}", boot_copy.host, boot_copy.port))?
    .run()
    .await
}
