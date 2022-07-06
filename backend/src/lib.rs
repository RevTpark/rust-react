#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod model;
pub mod schema;
pub mod header;
pub mod error;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_empty_json() -> String{
    let empty_json: String = serde_json::from_str::<serde_json::Value>("{}").unwrap().to_string();
    empty_json
}

pub fn get_json(message: String) -> serde_json::Value{
    let input: &str = &*format!(r#"{{"message": "{}"}}"#, message);
    let message_json: serde_json::Value = serde_json::from_str::<serde_json::Value>(input).unwrap();
    message_json
}