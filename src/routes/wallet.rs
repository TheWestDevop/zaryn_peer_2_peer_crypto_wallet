use actix_web::{ get, post, put, delete, web::{Data, Json, Path}, HttpResponse, Result};
use crate::utils::response::ZarynError;
use crate::models::wallet::{WalletInfo, Transfer};
use crate::models::state::{ AppState };
use crate::controllers::wallet::*;





#[get("")]
pub async fn get_wallets(state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    fetch_all_wallets(db).await
}

#[post("")]
pub async fn create_wallet(wallet_info: Json<WalletInfo>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return create_user_wallet(wallet_info,db).await;
}

#[get("/info/{public_key}")]
pub async fn get_wallet_info(Path(public_key): Path<String>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return fetch_wallet_info(public_key,db).await;
}


#[put("/transfer")]
pub async fn update_wallet(transaction_info: Json<Transfer>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return do_transfer(transaction_info,db).await;
}


#[delete("/remove/{wallet_address}")]
pub async fn delete_wallet(Path(wallet_address): Path<String>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return delete_this_wallet(wallet_address,db).await;
}
