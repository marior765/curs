#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
 
#[macro_use]
extern crate rocket;
extern crate diesel; 
extern crate rocket_contrib;
extern crate serde_derive;
extern crate sentry;

mod routes;

use rocket::ignite;

fn main() {
    let _guard = sentry::init("https://6f3d5558f98e4a1aa6f5d8e45705378d@sentry.io/1436182");
    sentry::integrations::panic::register_panic_handler();

    ignite().mount("/", routes![
        routes::start,
        routes::logout,
        routes::show_post,
        routes::create_post,
        routes::delete_post,
    ]).launch();
}