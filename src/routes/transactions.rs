use actix_web::{ get, web::{Data,  Path}, HttpResponse, Result};
use crate::utils::response::{ZarynError};
use crate::models::state::{ AppState };
use crate::controllers::transaction::*;



#[get("")]
pub async fn get_transactions(state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return get_all_transactions(db).await;
}

#[get("/{wallet_address}")]
pub async fn get_wallet_transactions(Path(wallet_address): Path<String>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return get_this_wallet_transaction(wallet_address,db).await;
}

#[get("/info/{transaction_address}")]
pub async fn get_transaction_info(Path(transaction_address): Path<String>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return get_this_transaction_info(transaction_address,db).await;
}

