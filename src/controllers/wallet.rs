use crate::actors::transaction::{CreateTransaction, UpdateTransaction};
use crate::actors::db::DBActor;
use crate::actors::wallet::{GetByWallet, Update};
use crate::utils::response::{ZarynError, ZarynMessage};
use actix::Addr;
use actix_web::{HttpResponse, Result};

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
            let total_cost: f64 = amount + 0.0001;
            let available_balance: f64 = data.amount.parse().unwrap();
            if available_balance >= total_cost {
                match update_wallet(sender_address.clone(), available_balance.to_string(), db).await
                {
                    Ok(_) => {
                        let _ = borrowed_db_state.send(CreateTransaction::new(
                            total_cost.to_string(),
                            sender_address.clone(),
                            receiver_wallet,
                            transaction_address,
                            transaction_signature,
                            transaction_type,
                            "0.0001".to_string(),
                            "Pending".to_string(),
                        ))
                        .await;

                        true
                    }
                    Err(_) => false,
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
            match update_wallet(wallet_address, available_balance.to_string(), db).await {
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
    let transaction_address = "transaction_address_here";
    let transaction_signature = "transaction_signature_here";

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
            sender_wallet_address,
            amount,
            db,
        )
        .await
    } else {
        Err(ZarynError::NotEnoughBalance)
    }
}
