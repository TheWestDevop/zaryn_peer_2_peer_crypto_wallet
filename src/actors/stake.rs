use crate::actix::{Handler, Message};
use crate::actors::db::DBActor;
use crate::diesel::prelude::*;
use crate::models::stake::*;
use crate::schema::stakes::dsl::{id,stake_signature, public_key, isactive, stakes};

#[derive(Message)]
#[rtype(result = "QueryResult<Stake>")]
pub struct Create {
    pub new_stake_from: String,
    pub new_stake_signature: String,
    pub new_public_key: String,
    pub new_amount: String,
    pub new_deadline: i64,
}
impl Create {
    pub fn this(
        new_stake_from: String,
        new_stake_signature: String,
        new_public_key: String,
        new_amount: String,
        new_deadline:i64
    ) -> Create {
        Create {
            new_stake_from,
            new_stake_signature,
            new_public_key,
            new_amount,
            new_deadline
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Stake>")]
pub struct Get {
    pub this_stake_signature: String,
    pub this_public_key: String,
}
impl Get {
    pub fn this(
        this_stake_signature: String,
        this_public_key: String,
    ) -> Get {
        Get {
            this_stake_signature,
            this_public_key,
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Stake>")]
pub struct Update {
    pub this_isactive: bool,
    pub this_stake_signature: String,
}
impl Update {
    pub fn this(this_isactive: bool, this_stake_signature: String) -> Update {
        Update {
            this_isactive,
            this_stake_signature,
        }
    }
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Stake>>")]
pub struct GetAllStakes;

impl Handler<Create> for DBActor {
    type Result = QueryResult<Stake>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        
        let new_stake = NewStake::new(
                msg.new_stake_from,
                msg.new_stake_signature,
                msg.new_public_key,
                msg.new_amount,
                msg.new_deadline,
            );

            match self.0.get() {
              Ok(result) => diesel::insert_into(stakes)
              .values(new_stake)
              .get_result::<Stake>(&result),
              _ => Err(diesel::result::Error::RollbackTransaction)
            }
            
    }
}

impl Handler<Get> for DBActor {
    type Result = QueryResult<Stake>;

    fn handle(&mut self, msg: Get, _: &mut Self::Context) -> Self::Result {

        match self.0.get() {
            Ok(result) => stakes
            .filter(
                stake_signature
                    .eq(msg.this_stake_signature)
                    .and(public_key.eq(msg.this_public_key)),
            )
            .get_result::<Stake>(&result),
            _ => Err(diesel::result::Error::RollbackTransaction)
          }
    }
}

impl Handler<Update> for DBActor {
    type Result = QueryResult<Stake>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {

        match self.0.get() {
            Ok(result) => diesel::update(stakes)
            .filter(stake_signature.eq(msg.this_stake_signature))
            .set(isactive.eq(msg.this_isactive))
            .get_result::<Stake>(&result),
            _ => Err(diesel::result::Error::RollbackTransaction)
        }
        
    }
}
impl Handler<GetAllStakes> for DBActor {
    type Result = QueryResult<Vec<Stake>>;

    fn handle(&mut self, _msg: GetAllStakes, _: &mut Self::Context) -> Self::Result {
        match self.0.get() {
            Ok(result) => stakes.order(id.desc()).get_results::<Stake>(&result),
            _ => Err(diesel::result::Error::RollbackTransaction)
        }

        
    }
}