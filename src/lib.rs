#[macro_use] pub extern crate diesel;
pub use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    PgConnection::establish(&db_url)
        .expect("Error connecting to database")
}