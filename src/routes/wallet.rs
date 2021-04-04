use actix_web::{ get, post, put, delete, web::{Data, Json, Path}, HttpResponse, Result};
use crate::utils::response::{ ListZarynResponse, ZarynResponse, ZarynError};
use crate::models::wallet::{WalletInfo,BalanceInfo};
use crate::models::state::{ AppState };
use crate::actors::wallet::{GetAllWallets, Create, Update, Delete, Get, Detail };





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
            Ok(Ok(data)) => Ok(ZarynResponse::success(true, Some(data), Some("Wallet Created Successfully".to_string()))),
            _ => Err(ZarynError::InternalError)
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
        Ok(Ok(data)) => Ok(ZarynResponse::success(true, Some(data), Some("Wallet Balance Updated Successfully".to_string()))),
        _ => Err(ZarynError::InternalError)
    }
}


#[put("/update/{wallet_address}")]
pub async fn update_wallet(Path(wallet_address): Path<String>, wallet_balance: Json<BalanceInfo>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();

    match db.send(Update::this(
                       wallet_balance.amount.to_string(), 
                       wallet_balance.user_wallet_value.to_string(),
                       wallet_address
                    )).await {
        Ok(Ok(data)) => Ok(ZarynResponse::success(true, Some(data), Some("Wallet Balance Updated Successfully".to_string()))),
        _ => Err(ZarynError::InternalError)
    }
}


#[delete("/remove/{wallet_address}")]
pub async fn delete_wallet(Path(wallet_address): Path<String>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    match db.send(Delete::this(wallet_address)).await {
        Ok(Ok(_)) => Ok(ZarynResponse::success(true, None, Some("Wallet Deleted Successfully".to_string()))),
        Ok(Err(_)) => Err(ZarynError::WalletNotFound),
        _ => Err(ZarynError::InternalError)
    }
}
