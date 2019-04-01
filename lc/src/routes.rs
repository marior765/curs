use rocket::response::{Redirect};
use db::*;
use self::models::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket_contrib::json::{Json};

// static conn: PgConnection = db::establishconnection();

#[get("/")]
pub fn start() -> &'static str { 
    "Hello world!" 
}

#[get("/post/<_id>")]
pub fn show_post(_id: i32) -> Json<String> {
    let conn: PgConnection = db::establish_connection();
    use self::schema::posts::dsl::*;

    let result = posts.find(_id)
                    .execute(&conn)
                    .expect("Error find post");

    println!("Displaying {} posts", result);
        
    Json("Success".to_string())
}

#[post("/create_post", data = "<post>")]
pub fn create_post(post: Json<Post>) -> &'static str {
	let conn: PgConnection  = db::establish_connection();
	use self::schema::posts::dsl::*;

    // let ex = Post::new(1, "privet".to_string(), "gsgsdg".to_string(), false);

	diesel::insert_into(posts)
		.values(&post.into_inner())
		.execute(&conn)
		.expect("Error creating post");

	"Post have been successfully created!"
}

#[post("/delete_post/<_id>")]
pub fn delete_post(_id: i32) -> &'static str {
    use self::schema::posts::dsl::*;
    let conn: PgConnection  = db::establish_connection();
    diesel::delete(posts.find(_id)).execute(&conn).is_ok();

    "Post have been successfully deleted!"
}

#[post("/logout")]
pub fn logout() {
    Redirect::to("/");
}