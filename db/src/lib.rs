#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate serde_derive;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://postgres:1651234789@localhost/postgres";

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
