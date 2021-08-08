use serde::{Serialize, Deserialize};
use crate::utils::response::{ZarynError};
use crate::utils::crypto::encode;
use crate::models::transaction::{Transaction};
use crate::actors::transaction::{GetAllWalletTransactions, GetTransaction, GetAllTransactions };
use actix::Addr;
use actix_web::HttpResponse;
use crate::actors::db::DBActor;


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


pub async fn get_all_transactions(db:Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
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

pub async fn get_this_wallet_transaction(wallet_address: String,db:Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
    let _key = encode(&wallet_address).await;

    match db.send(GetAllWalletTransactions::tranxaction(
        _key
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

pub async fn get_this_transaction_info(transaction_address: String,db:Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
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
