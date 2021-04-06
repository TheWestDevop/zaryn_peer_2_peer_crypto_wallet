use crate::actix::{Actor, SyncContext};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct DBActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DBActor {
    type Context = SyncContext<Self>;
}