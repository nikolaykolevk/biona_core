#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
pub extern crate diesel;
extern crate dotenv;
pub extern crate rocket_contrib;
pub extern crate serde;
pub extern crate serde_json;
pub extern crate chrono;

pub extern crate biona_macros;
pub mod models;
mod paths;

use diesel::{PgConnection, Connection};
pub use serde::Serialize;



//returns a connection to the db with authentication info from the .env file
pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}