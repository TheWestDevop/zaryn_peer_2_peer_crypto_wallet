use actix_web::{ get, post, put, delete, web::{Data, Json, Path}, HttpResponse, Result};
use crate::utils::response::{ ListZarynResponse, ZarynResponse, ZarynMessage, ZarynError};
use crate::models::wallet::{WalletInfo, Transfer};
use crate::models::state::{ AppState };
use crate::actors::wallet::{GetAllWallets, Create, Delete, Get, Detail };
use crate::controllers::wallet::*;





#[get("")]
pub async fn get_wallets(state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    match db.send(GetAllWallets).await {
        Ok(Ok(data)) => Ok(ListZarynResponse::success(true, Some(data), Some("All Wallets".to_string()))),
        _ => Err(ZarynError::InternalError)
    }
}
#[post("")]
pub async fn create_wallet(wallet_info: Json<WalletInfo>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();

    // println!("{:?}",new_wallet);

    if wallet_info.wallet_address.is_empty() || 
       wallet_info.private_key.is_empty() || 
       wallet_info.public_key.is_empty() ||
       wallet_info.amount.is_empty() ||
       wallet_info.user_wallet_value.is_empty()   {
        return Err(ZarynError::BadClientData)
    }

    match db.send(
        Get::this(
        wallet_info.wallet_address.to_string(),
        wallet_info.public_key.to_string(),
        wallet_info.private_key.to_string(),
        )
     ).await {
         Ok(Ok(_)) => Err(ZarynError::ErrorDuplicateWalletFound),
         Ok(Err(_)) => match db.send(
            Create::this(
            wallet_info.wallet_address.to_string(),
            wallet_info.private_key.to_string(),
            wallet_info.public_key.to_string(),
            wallet_info.amount.to_string(),
            wallet_info.user_wallet_value.to_string()
            )
         ).await {
            Ok(Ok(data)) => Ok(ZarynResponse::success(true, Some(data), "Wallet Created Successfully".to_string())),
            _ => Err(ZarynError::TransactionNotProcessed)
        },
         Err(_) => Err(ZarynError::InternalError)  
     }
     
}

#[get("/{public_key}")]
pub async fn get_wallet_info(Path(public_key): Path<String>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();

    match db.send(Detail::this(
                      public_key
                    )).await {
        Ok(Ok(data)) => Ok(ZarynResponse::success(true, Some(data), "Wallet Details".to_string())),
        _ => Err(ZarynError::InternalError)
    }
}


#[put("/transfer")]
pub async fn update_wallet(transaction_info: Json<Transfer>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    if  transaction_info.sender_private_key.is_empty() ||
        transaction_info.sender_public_key.is_empty() {
          return Err(ZarynError::ValidationError {  field: "private key and public key".to_string() });
    }
    match process_transfer(transaction_info.sender_wallet_address.clone(), transaction_info.receiver_wallet_address.clone(), transaction_info.amount.clone(), db).await {
        Ok(_) => Ok(ZarynMessage::success(true, "Transfer is been processed".to_string())),
        Err(_) => Err(ZarynError::TransactionNotProcessed),
    }
}


#[delete("/remove/{wallet_address}")]
pub async fn delete_wallet(Path(wallet_address): Path<String>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    match db.send(Delete::this(wallet_address)).await {
        Ok(Ok(_)) => Ok(ZarynMessage::success(true,"Wallet Deleted Successfully".to_string())),
        Ok(Err(_)) => Err(ZarynError::WalletNotFound),
        _ => Err(ZarynError::InternalError)
    }
}
