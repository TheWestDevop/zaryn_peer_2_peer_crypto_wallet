use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::transactions;
use chrono::NaiveDateTime;


#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub input_amount: String,
    pub output_amount: String,
    pub sender_wallet: String,
    pub receiver_wallet: String,
    pub transaction_address: String,
    pub transaction_signature: String,
    pub transaction_type: String,
    pub transaction_status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="transactions"]
pub struct NewTransaction {
    pub input_amount: String,
    pub output_amount: String,
    pub sender_wallet: String,
    pub receiver_wallet: String,
    pub transaction_address: String,
    pub transaction_signature: String,
    pub transaction_type: String,
    pub transaction_status: String,
}
impl NewTransaction {
   pub fn new(input_amount: String, output_amount: String, sender_wallet: String, receiver_wallet: String, transaction_address: String, transaction_signature: String, transaction_type: String, transaction_status: String ) -> NewTransaction {
      NewTransaction {
          input_amount,
          output_amount,
          sender_wallet,
          receiver_wallet,
          transaction_address,
          transaction_signature,
          transaction_type,
          transaction_status
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WalletInfo {
    pub input_amount: String,
    pub output_amount: String,
    pub wallet_address: String,
    pub transaction_signature: String,
    pub transaction_type: String,
    pub transaction_status: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct UpdateWalletInfo {
    pub input_amount: String,
    pub output_amount: String,
    pub wallet_address: String,
    pub transaction_signature: String,
    pub transaction_type: String,
    pub transaction_status: String,
}