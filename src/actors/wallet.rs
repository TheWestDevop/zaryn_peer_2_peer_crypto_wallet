use crate::actix::{Handler, Message};
use crate::actors::db::DBActor;
use crate::diesel::prelude::*;
use crate::models::wallet::*;
use crate::schema::wallets::dsl::{amount, id, wallet_signature, public_key, wallet_address, wallets};

#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Create {
    pub new_wallet_address: String,
    pub new_wallt_signature: String,
    pub new_public_key: String,
    pub default_amount: String,
}
impl Create {
    pub fn this(
        new_wallet_address: String,
        new_wallt_signature: String,
        new_public_key: String,
        default_amount: String,
    ) -> Create {
        Create {
            new_wallet_address,
            new_wallt_signature,
            new_public_key,
            default_amount,
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Get {
    pub user_wallet_address: String,
    pub user_public_key: String,
    pub user_wallt_signature: String,
}
impl Get {
    pub fn this(
        user_wallet_address: String,
        user_public_key: String,
        user_wallt_signature: String,
    ) -> Get {
        Get {
            user_wallet_address,
            user_public_key,
            user_wallt_signature,
        }
    }
}
#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct GetByWallet {
    pub user_wallet_address: String,
}
impl GetByWallet {
    pub fn this(user_wallet_address: String) -> GetByWallet {
        GetByWallet {
            user_wallet_address,
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Detail {
    pub user_public_key: String,
}
impl Detail {
    pub fn this(user_public_key: String) -> Detail {
        Detail { user_public_key }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Update {
    pub new_amount: String,
    pub user_wallet_address: String,
}
impl Update {
    pub fn this(new_amount: String, user_wallet_address: String) -> Update {
        Update {
            new_amount,
            user_wallet_address,
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Delete {
    pub user_wallet_address: String,
}
impl Delete {
    pub fn this(user_wallet_address: String) -> Delete {
        Delete {
            user_wallet_address,
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Wallet>>")]
pub struct GetAllWallets;

impl Handler<Create> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        
        let new_wallet = NewWallet::new(
                msg.new_wallet_address,
                msg.new_wallt_signature,
                msg.new_public_key,
                msg.default_amount,
                "none".to_string(),
            );

            match self.0.get() {
              Ok(result) => diesel::insert_into(wallets)
              .values(new_wallet)
              .get_result::<Wallet>(&result),
              _ => Err(diesel::result::Error::RollbackTransaction)
            }
            
    }
}
impl Handler<Get> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Get, _: &mut Self::Context) -> Self::Result {

        match self.0.get() {
            Ok(result) => wallets
            .filter(
                wallet_address
                    .eq(msg.user_wallet_address)
                    .and(public_key.eq(msg.user_public_key))
                    .and(wallet_signature.eq(msg.user_wallt_signature)),
            )
            .get_result::<Wallet>(&result),
            _ => Err(diesel::result::Error::RollbackTransaction)
          }
    }
}

impl Handler<GetByWallet> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: GetByWallet, _: &mut Self::Context) -> Self::Result {
        match self.0.get() {
            Ok(result) => wallets
            .filter(wallet_address.eq(msg.user_wallet_address))
            .order(id.desc())
            .get_result::<Wallet>(&result),
            _ => Err(diesel::result::Error::RollbackTransaction)
        }
        
    }
}

impl Handler<Detail> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Detail, _: &mut Self::Context) -> Self::Result {
        match self.0.get() {
            Ok(result) => wallets
            .filter(public_key.eq(msg.user_public_key))
            .get_result::<Wallet>(&result),
            _ => Err(diesel::result::Error::RollbackTransaction)
        }
    }
}

impl Handler<Update> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {

        match self.0.get() {
            Ok(result) => diesel::update(wallets)
            .filter(wallet_address.eq(msg.user_wallet_address))
            .set(amount.eq(msg.new_amount))
            .get_result::<Wallet>(&result),
            _ => Err(diesel::result::Error::RollbackTransaction)
        }
        
    }
}

impl Handler<Delete> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        match self.0.get() {
            Ok(result) => diesel::delete(wallets)
            .filter(wallet_address.eq(msg.user_wallet_address))
            .get_result::<Wallet>(&result),
            _ => Err(diesel::result::Error::RollbackTransaction)
        }   
    }
}

impl Handler<GetAllWallets> for DBActor {
    type Result = QueryResult<Vec<Wallet>>;

    fn handle(&mut self, _msg: GetAllWallets, _: &mut Self::Context) -> Self::Result {
        match self.0.get() {
            Ok(result) => wallets.order(id.desc()).get_results::<Wallet>(&result),
            _ => Err(diesel::result::Error::RollbackTransaction)
        }

        
    }
}
