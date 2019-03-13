#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate diesel;
extern crate dotenv;
extern crate serde;

pub mod connect;
pub mod models;
pub mod queries;
pub mod lib;
pub mod routes;

// use connect::establish_connection;
// use routes::hello_world;
// use lib::show_posts;
// use models::*;
// use std::io::{stdin};

fn main() {
    // let conn = establish_connection();
    // show_posts::<Post>(5, &conn);
    // let mut title = String::new();
    // stdin().read_line(&mut title).unwrap();
    rocket::ignite()
        .mount("/", routes![
            routes::hello_world, 
            routes::hadler, 
            routes::logout, 
            routes::new
        ])
        .launch();
    // println!("Hello, world!");
}
