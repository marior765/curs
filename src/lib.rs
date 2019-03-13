#[path="./models.rs"]
mod models;

// use diesel::prelude::*;
use diesel::pg::PgConnection;
use models::{NewPost};

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

pub fn create_post<'a>(_conn: &PgConnection, title: &'a str, body: &'a str) {
    // use schema::posts;

    let _new_post = NewPost { title, body };

    // _new_post


    // diesel::insert_into(posts::table)
    //     .values(&new_post)
    //     .get_result(conn)
    //     .expect("Error saving new post")
}


pub fn show_posts<T>(_n: u8, _conn: &PgConnection) {
    // let result = post.filter(published.eq(true))
    //     .limit(n)
    //     .load::<T>(_conn)
    //     .expect("Error loading posts");

    let result = ["privet","a vot i net"];

    println!("Display {} posts", result.len());
    for post in &result {
        println!("<>");
        println!("{}", post); // .title
        println!("----------\n");
        println!("{}", post); // .body
        println!("<>");
    }
}