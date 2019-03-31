#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::{Post, NewPost};
// use schema::{posts};
// use schema::posts::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
       .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn show_posts(_n: u8, _conn: &PgConnection) -> Post {
//     use schema::posts::dsl::*;

//     posts.filter(published.eq(true))
//         .limit(_n)
//         .load::<Post>(_conn)
//         .expect("Error loading post")
// }

pub fn create_post<'a>(_conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
        use schema::posts;

    let new_post = NewPost {
        title,
        body
    };

    diesel::insert_into(posts::table)
        .values(new_post)
        .get_result(_conn)
        .expect("Error saving new post")
}
