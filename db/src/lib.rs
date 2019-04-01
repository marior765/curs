#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate serde_derive;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {

    let database_url = "postgres://postgres:1651234789@localhost/postgres";
    
    PgConnection::establish(&database_url)
       .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}