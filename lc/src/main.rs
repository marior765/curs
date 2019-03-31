#[macro_use] extern crate diesel;

use db::*;
use self::models::*;
use self::schema::posts::dsl::*;
use diesel::prelude::*;

fn main() {
    let num = 10;
    let _conn = db::establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&_conn)
        .expect("Error loading posts");

}