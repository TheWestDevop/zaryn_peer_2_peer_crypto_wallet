use crate::actors::db::DBActor;
use crate::actix::Addr;


pub struct AppState {
    pub db: Addr<DBActor>
 }
 impl AppState {
     pub fn new(db: Addr<DBActor>) -> AppState {
         AppState {
              db
          }
      }
  }