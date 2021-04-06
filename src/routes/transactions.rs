use actix_web::{ get, web::{Data,  Path}, HttpResponse, Result};
use serde::{Serialize, Deserialize};
use crate::utils::response::{ZarynError};
use crate::models::transaction::{Transaction};
use crate::models::state::{ AppState };
use crate::actors::transaction::{GetAllWalletTransactions, GetTransaction, GetAllTransactions };

#[derive(Debug, Serialize, Deserialize, )]
struct Message {
    status:bool,
    data:Option<Transaction>,
    message:String
}

#[derive(Debug, Serialize, Deserialize, )]
struct List {
    status:bool,
    data:Option<Vec<Transaction>>,
    message:String
}

#[get("")]
pub async fn get_transactions(state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();
    match db.send(GetAllTransactions).await {
        Ok(Ok(data)) => Ok(
            HttpResponse::Ok().json(
                List {
                    status:true,
                    data:Some(data),
                    message:"All Transactions".to_string(),
               }
            )
        ),
        _ => Err(ZarynError::TransactionNotProcessed)
    }
}

#[get("/{wallet_address}")]
pub async fn get_wallet_transactions(Path(wallet_address): Path<String>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();

    match db.send(GetAllWalletTransactions::tranxaction(
        wallet_address
                    )).await {
        Ok(Ok(data)) => Ok(
            HttpResponse::Ok().json(
                List {
                    status:true,
                    data:Some(data),
                    message:"Wallet Transactions".to_string(),
               }
            )
        ),
        _ => Err(ZarynError::TransactionNotProcessed)
    }
}

#[get("/{transaction_address}")]
pub async fn get_transaction_info(Path(transaction_address): Path<String>, state: Data<AppState>) -> Result<HttpResponse, ZarynError> {
    let db = state.as_ref().db.clone();

    match db.send(GetTransaction::tranxaction(
        transaction_address
                    )).await {
        Ok(Ok(data)) => Ok(
            HttpResponse::Ok().json(
                Message {
                    status:true,
                    data:Some(data),
                    message:"Transaction Details".to_string(),
               }
            )
        ),
        _ => Err(ZarynError::TransactionNotFound)
    }
}

