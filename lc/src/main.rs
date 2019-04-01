#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
 
#[macro_use]
extern crate rocket;
extern crate diesel; 
extern crate rocket_contrib;
extern crate serde_derive;

mod routes;

use rocket::ignite;

fn main() {
    ignite().mount("/", routes![
        routes::start,
        routes::logout,
        routes::show_post,
        routes::create_post,
        routes::delete_post,
    ]).launch();
}