use crate::actix::{Handler, Message};
use crate::diesel::prelude::*;
use crate::models::transaction::*;
use crate::schema::transactions::dsl::*;
use crate::actors::db::DBActor;


#[derive(Message)]
#[rtype(result = "QueryResult<Transaction>")]
pub struct CreateTransaction {
    pub x_amount: String,
    pub x_sender_wallet: String,
    pub x_receiver_wallet: String,
    pub x_transaction_address: String,
    pub x_transaction_signature: String,
    pub x_transaction_type: String,
    pub x_transaction_fee: String,
    pub x_transaction_status: String,
}
impl CreateTransaction {
    pub fn new(
        x_amount: String,
        x_sender_wallet: String,
        x_receiver_wallet: String,
        x_transaction_address: String,
        x_transaction_signature: String,
        x_transaction_type: String,
        x_transaction_fee: String,
        x_transaction_status: String,
    ) -> CreateTransaction {
        CreateTransaction {
            x_amount,
            x_sender_wallet,
            x_receiver_wallet,
            x_transaction_address,
            x_transaction_signature,
            x_transaction_type,
            x_transaction_fee,
            x_transaction_status,
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Transaction>")]
pub struct GetTransaction {
    pub x_transaction_address: String,
}
impl GetTransaction {
    pub fn tranxaction(x_transaction_address: String) -> GetTransaction {
        GetTransaction {
            x_transaction_address,
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Transaction>>")]
pub struct GetAllWalletTransactions {
    pub wallet_address: String,
}
impl GetAllWalletTransactions {
    pub fn tranxaction(wallet_address: String) -> GetAllWalletTransactions {
        GetAllWalletTransactions { wallet_address }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Transaction>")]
pub struct UpdateTransaction {
    pub x_transaction_signature: String,
    pub x_transaction_status: String,
}
impl UpdateTransaction {
    pub fn tranxaction(x_transaction_signature: String, x_transaction_status: String) -> UpdateTransaction {
        UpdateTransaction {
            x_transaction_signature,
            x_transaction_status,
        }
    }
}
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Transaction>>")]
pub struct GetAllTransactions;


impl Handler<CreateTransaction> for DBActor {
    type Result = QueryResult<Transaction>;

    fn handle(&mut self, msg: CreateTransaction, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_transaction = NewTransaction::new(
            msg.x_amount,
            msg.x_sender_wallet,
            msg.x_receiver_wallet,
            msg.x_transaction_address,
            msg.x_transaction_signature,
            msg.x_transaction_status,
            msg.x_transaction_type,
            msg.x_transaction_fee,
        );

        diesel::insert_into(transactions)
            .values(new_transaction)
            .get_result::<Transaction>(&conn)
    }
}
impl Handler<GetTransaction> for DBActor {
    type Result = QueryResult<Transaction>;

    fn handle(&mut self, msg: GetTransaction, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        transactions
            .filter(transaction_address.eq(msg.x_transaction_address))
            .get_result::<Transaction>(&conn)
    }
}
impl Handler<GetAllWalletTransactions> for DBActor {
    type Result = QueryResult<Vec<Transaction>>;

    fn handle(&mut self, msg: GetAllWalletTransactions, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        transactions
            .filter(sender_wallet.eq(msg.wallet_address))
            .get_results::<Transaction>(&conn)
    }
}
impl Handler<UpdateTransaction> for DBActor {
    type Result = QueryResult<Transaction>;

    fn handle(&mut self, msg: UpdateTransaction, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(transactions)
            .filter(transaction_signature.eq(msg.x_transaction_signature))
            .set(transaction_status.eq(msg.x_transaction_status))
            .get_result::<Transaction>(&conn)
    }
}
impl Handler<GetAllTransactions> for DBActor {
    type Result = QueryResult<Vec<Transaction>>;

    fn handle(&mut self, _msg: GetAllTransactions, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        transactions.get_results::<Transaction>(&conn)
    }
}
