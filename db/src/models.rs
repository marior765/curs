use super::schema::*;
use rocket::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Insertable, FromForm)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

impl Post {

    pub fn new(id: i32, title: String, body: String, published: bool) -> Self {
        Post {
            id,
            title,
            body,
            published
        }
    }
}

// impl Post {

//     pub fn show_posts(_n: u8, _conn: &PgConnection) -> Post {
//         posts.filter(published.eq(true))
//             .limit(_n)
//             .load::<Post>(_conn)
//             .expect("Error loading post")
//     }

//     pub fn create_posts(post: Post, _conn: &PgConnection) -> Post {
//         diesel::insert_into(posts::table)
//             .values(&post)
//             .execute(_conn)
//             .expect("!");

//         posts::table.order().first(_conn).unwrap()
//     }
    
// }

// #[table_name = "heroes"]
// #[derive(Serialize, Deserialize, Queryable, Insertable)]
// pub struct Hero {
//     pub id: Option<i32>,
//     pub name: String,
//     pub identity: String,
//     pub hometown: String,
//     pub age: i32
// }


// impl Hero {
//     pub fn create(hero: Hero, connection: &MysqlConnection) -> Hero {
//         diesel::insert_into(heroes::table)
//             .values(&hero)
//             .execute(connection)
//             .expect("Error creating new hero");

//         heroes::table.order(heroes::id.desc()).first(connection).unwrap()
//     }

//     pub fn read(connection: &MysqlConnection) -> Vec<Hero> {
//         heroes::table.order(heroes::id.asc()).load::<Hero>(connection).unwrap()
//     }

//     pub fn update(id: i32, hero: Hero, connection: &MysqlConnection) -> bool {
//         diesel::update(heroes::table.find(id)).set(&hero).execute(connection).is_ok()
//     }

//     pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
//         diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
//     }
// }