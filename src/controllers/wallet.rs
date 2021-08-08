use crate::actors::transaction::{CreateTransaction, UpdateTransaction};
use crate::actors::db::DBActor;
use crate::utils::{response::*,crypto::{encode}};
use crate::models::wallet::{WalletInfo, Transfer};
use crate::actors::wallet::{GetAllWallets, GetByWallet,Update, Create, Delete, Get, Detail };
use actix::Addr;
use actix_web::{HttpResponse, Result,web::Json};
use chrono::prelude::*;


async fn is_wallet_capable(
    sender_address: String,
    receiver_wallet: String,
    transaction_address: String,
    transaction_signature: String,
    transaction_type: String,
    amount: f64,
    db: Addr<DBActor>,
) -> bool {
    let address = &sender_address;
    let borrowed_db_state = db.clone();
    let balance: bool = match borrowed_db_state
        .send(GetByWallet::this(address.to_string()))
        .await
    {
        Ok(Ok(data)) => {
            // println!("{:?}",data);
            let total_cost: f64 = amount + 0.000001;
            // println!("amount to withdraw from wallet is {:?}",total_cost);
            let available_balance: f64 = data.amount.parse().unwrap();
            // println!("available wallet balance is {:?}",available_balance);
            if available_balance >= total_cost {
                let new_wallet_balance = available_balance - total_cost;
                // println!("new wallet balance is {:?}",new_wallet_balance);
                match update_wallet(sender_address.clone(), new_wallet_balance.to_string(), db).await
                {
                    Ok(_) => {
                        let _ = borrowed_db_state.send(CreateTransaction::new(
                            total_cost.to_string(),
                            sender_address.clone(),
                            receiver_wallet,
                            transaction_address,
                            transaction_signature,
                            transaction_type,
                            "0.000001".to_string(),
                            "Pending".to_string(),
                        ))
                        .await;

                       

                        return true;
                    }
                    Err(_) => {
                        let _ = borrowed_db_state.send(UpdateTransaction::tranxaction(
                            transaction_signature,
                            "Failed".to_string(),
                        ))
                        .await;
                        return false;
                    },
                };
            }
            return false;
        }
        Ok(Err(_)) => false,
        Err(_) => false,
    };
    balance
}

pub async fn transfer(
    transaction_signature: String,
    wallet_address: String,
    amount: String,
    db: Addr<DBActor>,
) -> Result<HttpResponse, ZarynError> {
    let owner_address = &wallet_address;
    let borrowed_db_state = db.clone();
    match db.send(GetByWallet::this(owner_address.to_string())).await {
        Ok(Ok(data)) => {
            let mut available_balance: f64 = data.amount.parse().unwrap();
            let update_amount: f64 = amount.parse().unwrap_or_default();
            available_balance = available_balance + update_amount;
            match update_wallet(wallet_address, available_balance.to_string(), db.clone()).await {
                Ok(result) => {
                    let _ = borrowed_db_state.send(UpdateTransaction::tranxaction(
                        transaction_signature,
                        "Completed".to_string(),
                    ))
                    .await;

                    

                    return Ok(result);
                }
                Err(error) => {
                    let _ = borrowed_db_state.send(UpdateTransaction::tranxaction(
                        transaction_signature,
                        "Failed".to_string(),
                    ))
                    .await;
                    // let _ = update_wallet(sender_address, amount, db.clone()).await;
                    return Err(error);
                }
            }
        }
        Ok(Err(_)) => Err(ZarynError::WalletNotFound),
        Err(_) => Err(ZarynError::InternalError),
    }
}

pub async fn update_wallet(
    wallet_address: String,
    amount: String,
    db: Addr<DBActor>,
) -> Result<HttpResponse, ZarynError> {
    match db.send(Update::this(amount, wallet_address)).await {
        Ok(Ok(_)) => Ok(ZarynMessage::success(
            true,
            "Wallet Balance Updated Successfully".to_string(),
        )),
        _ => Err(ZarynError::InternalError),
    }
}

async fn process_transfer(
    sender_wallet_address: String,
    receiver_wallet_address: String,
    amount: String,
    db: Addr<DBActor>,
) -> Result<HttpResponse, ZarynError> {
    let sender_wallet = &sender_wallet_address;
    let receiver_wallet = &receiver_wallet_address;
    let transfer_amount = &amount;
    let borrowed_db_state = db.clone();
    let transaction_info = format!("sender address:{:?},  \n\
                                            receiver address:{:?}, \n\
                                            amount:{:?}, \n\
                                            timestamp:{:?} 
                                            ",
                                        &sender_wallet_address, 
                                        &receiver_wallet_address,
                                        &amount,
                                        &Utc::today()
                                        );
    let data_address =    format!("sender address:{:?},  \n\
                                         timestamp:{:?} 
                                        ",
                                        &sender_wallet_address, 
                                        &Utc::today()
                                        );
    let transaction_address = encode(&data_address).await;
    let transaction_signature = encode(&transaction_info).await;

    if is_wallet_capable(
        sender_wallet.to_string(),
        receiver_wallet.to_string(),
        transaction_address.to_string(),
        transaction_signature.to_string(),
        "Peer_2_Peer_Transfer".to_string(),
        transfer_amount.parse().unwrap_or_default(),
        borrowed_db_state,
    )
    .await
    {
        transfer(
            transaction_signature.clone().to_string(),
            receiver_wallet_address,
            amount,
            db,
        )
        .await



    } else {
        let _ = db.clone().send(UpdateTransaction::tranxaction(
            transaction_signature.clone().to_string(),
            "Failed".to_string(),
        ))
        .await;
        Err(ZarynError::NotEnoughBalance)
    }
}

pub async fn create_user_wallet(wallet_info: Json<WalletInfo>, db:Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
    let new_secure_wallet_signature = encode(&wallet_info.wallet_signature.to_string()).await;
    let new_secure_public_key = encode(&wallet_info.public_key.to_string()).await;
    let new_secure_address = encode(&wallet_info.wallet_address.to_string()).await;
    match db.send(
        Get::this(
        new_secure_address.to_string(),
        new_secure_public_key.to_string(),
        new_secure_wallet_signature.to_string(),
        )
     ).await {
         Ok(Ok(_)) => Err(ZarynError::ErrorDuplicateWalletFound),
         Ok(Err(_)) => match db.send(
            Create::this(
            new_secure_address,
            new_secure_wallet_signature,
            new_secure_public_key,
            "0".to_string(),
            )
         ).await {
            Ok(Ok(data)) => Ok(ZarynWalletResponse::success(true, Some(data), "Wallet Created Successfully".to_string())),
            _ => Err(ZarynError::InternalError)
        },
         Err(_) => Err(ZarynError::InternalError)  
     } 
}

pub async fn fetch_all_wallets(db:Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
    match db.send(GetAllWallets).await {
        Ok(Ok(data)) => Ok(ListZarynWalletResponse::success(true, Some(data), Some("All Wallets".to_string()))),
        _ => Err(ZarynError::InternalError)
    }
}

pub async fn fetch_wallet_info(public_key: String,db:Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
    let _key = encode(&public_key.to_string()).await;
    match db.send(Detail::this(
                      _key
                    )).await {
        Ok(Ok(data)) => Ok(ZarynWalletResponse::success(true, Some(data), "Wallet Details".to_string())),
        _ => Err(ZarynError::WalletNotFound)
    }
}

pub async fn do_transfer(transaction_info: Json<Transfer>, db:Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
    
    let pri_key = encode(&transaction_info.sender_wallet_signature.clone()).await;
    let pub_key = encode(&transaction_info.sender_public_key.clone()).await;
    let sender_wallet = encode(&transaction_info.sender_wallet_address.clone()).await;
    let receiver_wallet = encode(&transaction_info.receiver_wallet_address.clone()).await;

    
    // 
    match db.clone()
    .send(GetByWallet::this(sender_wallet.clone()))
    .await
    {
        Ok(Ok(wallet)) => {
            if wallet.wallet_signature.eq(&pri_key.to_string().clone()) && wallet.public_key.eq(&pub_key.to_string().clone())  {
                match process_transfer(sender_wallet.clone(), receiver_wallet, transaction_info.amount.clone(), db).await {
                    Ok(_) => Ok(ZarynMessage::success(true, "Transfer completed".to_string())),
                    Err(e) => Err(e),
                }
            }else{
                return Err(ZarynError::ValidationError {  field: "invalid authorization information".to_string() });
            }
            

        },
        Ok(Err(_)) => Err(ZarynError::WalletNotFound),
        Err(_) => Err(ZarynError::WalletNotFound), 
    }
}

pub async fn delete_this_wallet(wallet_address: String, db:Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
    let address = encode(&wallet_address).await;
    match db.send(Delete::this(address)).await {
        Ok(Ok(_)) => Ok(ZarynMessage::success(true,"Wallet Deleted Successfully".to_string())),
        Ok(Err(_)) => Err(ZarynError::WalletNotFound),
        _ => Err(ZarynError::InternalError)
    }
}

