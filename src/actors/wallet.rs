use crate::actix::{ Handler, Message};
use crate::diesel::prelude::*;
use crate::models::wallet::*;
use crate::schema::wallets::dsl::{wallet_address, public_key, private_key, amount,  wallets};
use crate::actors::db::DBActor;



#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Create {
    pub new_wallet_address: String,
    pub new_private_key: String,
    pub new_public_key: String,
    pub default_amount: String,
    pub user_wallet_value: String,
}
impl Create {
    pub fn this(
        new_wallet_address: String, 
        new_private_key: String, 
        new_public_key: String, 
        default_amount: String, 
        user_wallet_value: String
    ) -> Create {
        Create {
            new_wallet_address,
            new_private_key,
            new_public_key,
            default_amount,
            user_wallet_value
          }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Get {
    pub user_wallet_address: String,
    pub user_public_key: String,
    pub user_private_key: String,

}
impl Get {
    pub fn this( user_wallet_address: String, user_public_key: String, user_private_key: String ) -> Get {
        Get { user_wallet_address, user_public_key, user_private_key  }
    }
}
#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct GetByWallet {
    pub user_wallet_address: String,

}
impl GetByWallet {
    pub fn this( user_wallet_address: String) -> GetByWallet {
        GetByWallet { user_wallet_address }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Detail {
    pub user_public_key: String,

}
impl Detail {
    pub fn this( user_public_key: String ) -> Detail {
        Detail { user_public_key }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Update {
    pub new_amount: String,
    pub user_wallet_address: String
}
impl Update {
    pub fn this(new_amount: String, user_wallet_address: String) -> Update {
        Update {
            new_amount,
            user_wallet_address
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Wallet>")]
pub struct Delete {
    pub user_wallet_address: String,
}
impl Delete {
   pub  fn this(user_wallet_address:String) -> Delete  {
        Delete{
            user_wallet_address
        }
    }
}



#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Wallet>>")]
pub struct GetAllWallets;



impl Handler<Create> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_wallet = NewWallet::new(
                                    msg.new_wallet_address,
                                    msg.new_private_key,
                                    msg.new_public_key,
                                    msg.default_amount,
                                    msg.user_wallet_value
                                );

        diesel::insert_into(wallets)
            .values(new_wallet)
            .get_result::<Wallet>(&conn)
    }
}
impl Handler<Get> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Get, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        wallets.filter(
            wallet_address.eq(msg.user_wallet_address)
                          .and(public_key.eq(msg.user_public_key))
                          .and(private_key.eq(msg.user_private_key))
                    )
            .get_result::<Wallet>(&conn)
    }
}

impl Handler<GetByWallet> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: GetByWallet, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        wallets.filter(
            wallet_address.eq(msg.user_wallet_address)
                    )
            .get_result::<Wallet>(&conn)
    }
}

impl Handler<Detail> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Detail, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        wallets.filter(public_key.eq(msg.user_public_key))
            .get_result::<Wallet>(&conn)
    }
}

impl Handler<Update> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(wallets)
            .filter(wallet_address.eq(msg.user_wallet_address))
            .set(amount.eq(msg.new_amount))
            .get_result::<Wallet>(&conn)
    }
}

impl Handler<Delete> for DBActor {
    type Result = QueryResult<Wallet>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(wallets)
            .filter(wallet_address.eq(msg.user_wallet_address))
            .get_result::<Wallet>(&conn)
    }
}

impl Handler<GetAllWallets> for DBActor {
    type Result = QueryResult<Vec<Wallet>>;

    fn handle(&mut self, _msg: GetAllWallets, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        wallets.get_results::<Wallet>(&conn)
    }
}
