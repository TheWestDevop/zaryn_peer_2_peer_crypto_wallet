use crate::actors::{db::DBActor,wallet::Create};
use crate::utils::db::{get_pool, run_migrations};

use actix::{Addr, SyncArbiter};
use dotenv::dotenv;
use std::env;
use tracing_subscriber::EnvFilter;


#[derive(Clone)]
pub struct Bootstrap {
    
       pub host: String,
        pub port: String,
        pub workers: usize,
        pub db: Addr<DBActor>,
    
}



pub async fn boot() -> Bootstrap {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let env_value = get_env().await;

    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db = SyncArbiter::start(env_value.3, move || DBActor(pool.clone()));

    db.do_send(
        Create::this(
        "Zaryn_Staking_Pool".to_string(),
        "bbca5c9bbc07281bf78a1f4ee5c01925ea5494d98cbe5f9cda91d569cc521fa7".to_string(),
        "0b72eb362544561c4f196b3ddc6b9846aa6baab29e4838e95be8897cd84f7ac4".to_string(),
        "0".to_string(),
        )
     );


    Bootstrap {
        host: env_value.0,
        port: env_value.1,
        workers: env_value.2,
        db:db.clone(),
    }
}
 pub async fn get_env() -> (String, String, usize, usize) {
    let host = env::var("HOST").expect("Error Host");
    let port = env::var("PORT").expect("Error PORT");

    println!("Starting server at http://{}:{}/", &host, &port);

    let env_worker = env::var("WORKERS").expect("WORKERS not specified");
    let workers = match env_worker.trim().parse::<u8>() {
        Ok(e) => e,
        _ => 5,
    };

    let db_threads = env::var("DB_THREADS").expect("DB_THREADS not specified");
    let threads = match db_threads.trim().parse::<u8>() {
        Ok(e) => e,
        _ => 5,
    };

    (host, port, workers.into(), threads.into())
}
