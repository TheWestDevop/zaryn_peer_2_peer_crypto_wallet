use crate::actors::db::DBActor;
use crate::actors::stake::*;
use crate::actors::transaction::{CreateTransaction, UpdateTransaction};
use crate::actors::wallet::GetByWallet;
use crate::controllers::wallet::{update_wallet,transfer};
use crate::models::stake::{StakeInfo, WithdrawStake};
use crate::utils::{crypto::encode, response::*};
use actix::Addr;
use actix_web::{web::Json, HttpResponse, Result};
use chrono::prelude::*;

async fn is_stake_possible(
    sender_address: String,
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
            let available_balance: f64 = data.amount.parse().unwrap();
            if available_balance >= amount {
                let new_wallet_balance = available_balance - amount;
                match update_wallet(sender_address.clone(), new_wallet_balance.to_string(), db)
                    .await
                {
                    Ok(_) => {
                        let _ = borrowed_db_state
                            .send(CreateTransaction::new(
                                amount.to_string(),
                                sender_address.clone(),
                                "Staking Pool".to_string(),
                                transaction_address,
                                transaction_signature,
                                transaction_type,
                                "0.000000".to_string(),
                                "Pending".to_string(),
                            ))
                            .await;

                        return true;
                    }
                    Err(_) => {
                        let _ = borrowed_db_state
                            .send(UpdateTransaction::tranxaction(
                                transaction_signature,
                                "Failed".to_string(),
                            ))
                            .await;
                        return false;
                    }
                };
            }
            return false;
        }
        Ok(Err(_)) => false,
        Err(_) => false,
    };
    balance
}

async fn process_stake(
    sender_wallet_address: String,
    new_secure_signature: String,
    new_secure_public_key: String,
    deadline: i64,
    amount: String,
    db: Addr<DBActor>,
) -> Result<HttpResponse, ZarynError> {
    let sender_wallet = &sender_wallet_address;
    let transfer_amount = &amount;
    let borrowed_db_state = db.clone();
    let transaction_info = format!(
        "sender address:{:?},  \n\
                                            amount:{:?}, \n\
                                            timestamp:{:?} 
                                            ",
        &sender_wallet_address,
        &amount,
        &Utc::today()
    );
    let data_address = format!(
        "sender address:{:?},  \n\
                                         timestamp:{:?} 
                                        ",
        &sender_wallet_address,
        &Utc::today()
    );
    let transaction_address = encode(&data_address).await;
    let transaction_signature = encode(&transaction_info).await;

    if is_stake_possible(
        sender_wallet.to_string(),
        transaction_address.to_string(),
        transaction_signature.to_string(),
        "Peer_2_Peer_Transfer".to_string(),
        transfer_amount.parse().unwrap_or_default(),
        borrowed_db_state,
    )
    .await
    {
        match transfer(
            transaction_signature.clone().to_string(),
            "Zaryn_Staking_Pool".to_string(),
            amount.clone().to_string(),
            db.clone(),
        )
        .await {
            Ok(_) => match db
                    .send(Create::this(
                        sender_wallet_address,
                        new_secure_signature,
                        new_secure_public_key,
                        amount,
                        deadline,
                    ))
                .await
                    {
                        Ok(Ok(data)) => Ok(ZarynStakeResponse::success(
                            true,
                            Some(data),
                            "Stake Added Successfully".to_string(),
                        )),
                        _ => Err(ZarynError::InternalError),
                    }
            _ => Err(ZarynError::TransactionNotProcessed)
        }
        
    } else {
        let _ = db
            .clone()
            .send(UpdateTransaction::tranxaction(
                transaction_signature.clone().to_string(),
                "Failed".to_string(),
            ))
            .await;
        Err(ZarynError::NotEnoughBalance)
    }
}

pub async fn create_user_stake(
    stake_info: Json<StakeInfo>,
    db: Addr<DBActor>,
) -> Result<HttpResponse, ZarynError> {
    let stake_signature = format!(
        "sender wallet address:{:?}, \n\
                                         public key:{:?}, \n\
                                         amount:{:?}, \n\
                                         timestamp:{:?} 
                                        ",
        stake_info.stake_from.clone(),
        stake_info.public_key.clone(),
        &stake_info.amount,
        &Utc::today()
    );
    let new_secure_signature = encode(&stake_signature).await;
    let new_secure_public_key = encode(&stake_info.public_key.to_string()).await;
    match db
        .send(Get::this(
            new_secure_signature.to_string(),
            new_secure_public_key.to_string(),
        ))
        .await
    {
        Ok(Ok(_)) => Err(ZarynError::ErrorDuplicateStakeTransactionFound),
        Ok(Err(_)) => {
            process_stake(
                stake_info.stake_from.to_string(),
                new_secure_signature.to_string(),
                new_secure_public_key.to_string(),
                stake_info.deadline,
                stake_info.amount.to_string(),
                db,
            )
            .await
        }
        Err(_) => Err(ZarynError::InternalError),
    }
}
pub async fn fetch_all_stakes(db: Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
    match db.send(GetAllStakes).await {
        Ok(Ok(data)) => Ok(ListZarynStakeResponse::success(
            true,
            Some(data),
            Some("All Stakes".to_string()),
        )),
        _ => Err(ZarynError::StakeNotFound),
    }
}
pub async fn fetch_stake_info(
    stake_signature: String,
    key: String,
    db: Addr<DBActor>,
) -> Result<HttpResponse, ZarynError> {
    match db.send(Get::this(stake_signature, key)).await {
        Ok(Ok(data)) => Ok(ZarynStakeResponse::success(
            true,
            Some(data),
            "Stake Details".to_string(),
        )),
        _ => Err(ZarynError::StakeNotFound),
    }
}

async fn transfer_stake(receiver_wallet:String, stake_signature:String, amount: String, db: Addr<DBActor>) -> Result<HttpResponse, ZarynError> {
    let borrowed_db_state = db.clone();
    let transaction_info = format!("sender address:{:?},  \n\
                                            receiver address:{:?}, \n\
                                            amount:{:?}, \n\
                                            timestamp:{:?} 
                                            ",
                                        &"Zaryn_Staking_Pool".to_string(), 
                                        &receiver_wallet,
                                        &amount,
                                        &Utc::today()
                                        );
    let data_address =    format!("sender address:{:?},  \n\
                                         timestamp:{:?} 
                                        ",
                                        &"Zaryn_Staking_Pool".to_string(), 
                                        &Utc::today()
                                        );
    let transaction_address = encode(&data_address).await;
    let transaction_signature = encode(&transaction_info).await;
    borrowed_db_state.do_send(
        CreateTransaction::new(
        amount.to_string(),
        "Zaryn_Staking_Pool".to_string(),
        receiver_wallet,
        transaction_address,
        transaction_signature.clone().to_string(),
        "Stake Withdraw".to_string(),
        "0.000000".to_string(),
        "Pending".to_string(),
    ));

       match transfer(
        transaction_signature.clone().to_string(),
        "Zaryn_Staking_Pool".to_string(),
        amount.clone().to_string(),
        db.clone(),
    )
    .await {
        Ok(result) => {
                db.do_send(Update::this(this_isactive:'r#false', this_stake_signature:stake_signature));
                return Ok(
                    ZarynStakeResponse::success(
                        true,
                        None,
                        "Zaryn Withdraw is been processed".to_string(),
                    )
                );
        },
        _ => Err(ZarynError::TransactionNotProcessed)
    }
}

pub async fn withdraw_stake(
    stake_info: Json<WithdrawStake>,
    db: Addr<DBActor>
) -> Result<HttpResponse, ZarynError> {
    match db
    .send(Get::this(
        stake_info.stake_signature.to_string(),
        stake_info.public_key.to_string(),
    ))
    .await
{
    Ok(Ok(data)) => {
        transfer_stake(
            stake_info.stake_from.to_string(),
            stake_info.stake_signature.to_string(),
            data.amount,
            db,
        )
        .await
    },
    Ok(Err(_)) => Err(ZarynError::StakeNotFound),
    Err(_) => Err(ZarynError::InternalError),
}
}
