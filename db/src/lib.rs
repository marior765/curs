#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate serde_derive;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish("postgres://postgres:1651234789@localhost/postgres")
       .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn create_post<'a>(_conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
//         use schema::posts;

//     let new_post = NewPost {
//         title,
//         body
//     };

//     diesel::insert_into(posts::table)
//         .values(new_post)
//         .get_result(_conn)
//         .expect("Error saving new post")
// }
