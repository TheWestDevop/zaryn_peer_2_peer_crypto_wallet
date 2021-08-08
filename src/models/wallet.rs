use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::wallets;
use chrono::NaiveDateTime;



#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Wallet {
    pub id: i32,
    pub wallet_address: String,
    pub wallet_signature: String,
    pub public_key: String,
    pub amount: String,
    pub wallet_value: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="wallets"]
pub struct NewWallet {
    pub wallet_address: String,
    pub wallet_signature: String,
    pub public_key: String,
    pub amount: String,
    pub wallet_value: String,
}
impl NewWallet {
   pub fn new(wallet_address: String, wallet_signature: String, public_key: String, amount: String, wallet_value: String ) -> NewWallet {
      NewWallet {
          wallet_address,
          wallet_signature,
          public_key,
          amount,
          wallet_value
        }
    }
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct WalletInfo {
    pub wallet_address: String,
    pub wallet_signature: String,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceInfo {
    pub amount: String,
    pub user_wallet_value: String,
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct Transfer {
    pub sender_wallet_address: String,
    pub receiver_wallet_address: String,
    pub sender_wallet_signature: String,
    pub sender_public_key: String,
    pub amount: String,
}

