use rocket::http::RawStr;
use rocket::response::{Redirect, status, NamedFile};
use std::path::{Path, PathBuf};
use rocket::State;

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

pub struct AssetsDir(String);

#[get("/<asset..>")]
pub fn assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
}