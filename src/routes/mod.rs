use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket::response::status;

#[get("/")]
pub fn hello_world() -> &'static str { 
    "Hello world!" 
}

// #[post("/post/<name>")]
// pub fn post(name: String) {
//     m = name;
// }

#[post("/logout")]
pub fn logout() {
    Redirect::to("/");
}

#[get("/hello/<name>")]
pub fn hadler(name: &RawStr) -> String {
    format!("Hello, {}", name.as_str())
}

#[post("/<id>")]
pub fn new(id: usize) -> status::Accepted<String> {
    status::Accepted(Some(format!("id: '{}'", id)))
}