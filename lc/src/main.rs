#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate rocket_contrib;
extern crate sentry;
extern crate serde_derive;
extern crate tokio;

mod routes;

use rocket::ignite;
use tokio::{io::copy, net::TcpListener, prelude::*};

fn rocket_execute() {
    ignite()
        .mount(
            "/",
            routes![
                routes::start,
                routes::logout,
                routes::show_post,
                routes::create_post,
                routes::delete_post,
            ],
        )
        .launch();
}

fn tokio_execute() {
    let addr = "localhost:8000".parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("Unable to bind TCP listener");

    let server = listener
        .incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|socket| {
            let (reader, writer) = socket.split();

            let bytes_copied = copy(reader, writer);

            let handle_conn = bytes_copied
                .map(|amt| println!("wrote {:?} bytes", amt))
                .map_err(|e| eprintln!("IO error {:?}", e));

            tokio::spawn(handle_conn)
        });

    tokio::run(server);
}

fn main() {
    let _guard = sentry::init("https://6f3d5558f98e4a1aa6f5d8e45705378d@sentry.io/1436182");
    sentry::integrations::panic::register_panic_handler();

    rocket_execute();
    // tokio_execute();
}
