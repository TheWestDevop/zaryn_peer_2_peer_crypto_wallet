use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::stakes;
use chrono::{NaiveDateTime,Duration,Utc};



#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Stake {
    pub id: i32,
    pub stake_from: String,
    pub stake_signature: String,
    pub public_key: String,
    pub amount: String,
    pub withdraw_day: Option<NaiveDateTime>,
    pub isactive: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="stakes"]
pub struct NewStake {
    pub stake_from: String,
    pub stake_signature: String,
    pub public_key: String,
    pub amount: String,
    pub withdraw_day: Option<NaiveDateTime>,
    pub isactive:bool
}
impl NewStake {
   pub fn new(stake_from: String, stake_signature: String, public_key: String, amount: String, deadline: i64) -> NewStake {
    let days;
       if deadline < 15 {
           days =  Utc::now() + Duration::days(137);
       }else {
           days = Utc::now() + Duration::days(15)
       }
      NewStake {
          stake_from,
          stake_signature,
          public_key,
          amount,
          withdraw_day: Some(days.naive_utc()),
          isactive:true
        }
    }
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct StakeInfo {
    pub stake_from: String,
    pub stake_signature: String,
    pub public_key: String,
    pub amount: String,
    pub deadline: i64
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct WithdrawStake {
    pub stake_from: String,
    pub stake_signature: String,
    pub public_key: String,
    pub deadline: i64
}