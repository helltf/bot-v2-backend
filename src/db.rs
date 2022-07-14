<<<<<<<< HEAD:src/db.rs
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn create_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
========
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
>>>>>>>> e51bd76b02790027073bbc848775f8ace0dc3e3e:src/lib.rs
