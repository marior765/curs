use rocket::response::{Redirect};
// use rocket::http::Cookies;
use db::*;
use self::models::*;
use diesel::pg::PgConnection;
use rocket_contrib::json::{Json};

// #[get("/")]
// fn index(cookies: Cookies) -> Option<String> {
//     cookies.get("message")
//         .map(|value| format!("Message: {}", value))
// }

#[get("/")]
fn index() -> &'static str {
    "Hello, Denys!"
}

#[get("/post/<_id>")]
pub fn show_post(_id: i32) -> Json<String> {
    let conn: PgConnection = db::establish_connection();
    Post::show_single_post(_id, &conn);
        
    Json("Success".to_string())
}

#[post("/create_post", data = "<post>")]
pub fn create_post(post: Json<Post>) -> &'static str {
	let conn: PgConnection  = db::establish_connection();
    Post::create_posts(post.into_inner(), &conn)
}

#[post("/delete_post/<_id>")]
pub fn delete_post(_id: i32) -> &'static str {
    let conn: PgConnection  = db::establish_connection();
    Post::delete_post(_id, &conn)
}

#[post("/logout")]
pub fn logout() {
    Redirect::to("/");
}