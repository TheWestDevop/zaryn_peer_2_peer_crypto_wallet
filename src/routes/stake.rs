use actix_web::{ get, post, put, web::{Data, Json, Path}, HttpResponse, Result};
use crate::utils::response::ZarynError;
use crate::models::stake::{StakeInfo, WithdrawStake};
use crate::models::state::{ AppState };
use crate::controllers::stake::*;

#[get("")]
pub async fn get_stakes(state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    fetch_all_stakes(db).await
}

#[post("")]
pub async fn create_stake(stake_info: Json<StakeInfo>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return create_user_stake(stake_info,db).await;
}

#[put("/withdraw")]
pub async fn transfer_my_stake(withdraw_info: Json<WithdrawStake>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return withdraw_stake(withdraw_info,db).await;
}

#[get("/info/{stake_signature}/{key}")]
pub async fn get_stake_info(Path(stake_signature): Path<String>, Path(key): Path<String>,state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    return fetch_stake_info(stake_signature,key,db).await;
}

