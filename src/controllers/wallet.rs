use crate::actors::transaction::{CreateTransaction, UpdateTransaction};
use crate::actors::db::DBActor;
use crate::actors::wallet::{GetByWallet, Update};
use crate::utils::{response::{ZarynError, ZarynMessage},crypto};
use actix::Addr;
use actix_web::{HttpResponse, Result};
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

async fn transfer(
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

                    return Ok(result)
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

async fn update_wallet(
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

pub async fn process_transfer(
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
    let transaction_address = crypto::encode(&data_address).await;
    let transaction_signature = crypto::encode(&transaction_info).await;

    if is_wallet_capable(
        sender_wallet.to_string(),
        receiver_wallet.to_string(),
        transaction_address.to_string(),
        transaction_signature.to_string(),
        "Withdraw".to_string(),
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
