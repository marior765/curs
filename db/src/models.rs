use super::schema::*;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// #[derive(Queryable, Insertable, FromForm)]
// #[table_name="posts"]
// pub struct NewPost {
//     pub title: String,
//     pub body: String,
// }

impl Post {

    pub fn show_single_post(_id: i32, _conn: &PgConnection) {
        use super::schema::posts::dsl::*;
        let result = posts.find(_id)
                        .execute(_conn)
                        .expect("Error find post");

        println!("Displaying {} posts", result);

    }

    pub fn show_posts(_n: i32, _conn: &PgConnection) -> Vec<Post> {
        use super::schema::posts::dsl::*;

        posts.filter(published.eq(true))
            .limit(i64::from(_n))
            .load::<Post>(_conn)
            .expect("Error loading post")
    }

    pub fn create_posts(post: Post, _conn: &PgConnection) -> &'static str {
        diesel::insert_into(posts::table)
            .values(&post)
            .execute(_conn)
            .expect("!");

        "Post has been created"
    }

    pub fn update_post(_id: i32, post: Post, _conn: &PgConnection) -> &'static str {
        use super::schema::posts::dsl::*;
        diesel::update(posts.find(_id)).set(&post).execute(_conn).expect("Error updating post");

        "Post has been updated"
    }

    pub fn delete_post(_id: i32, _conn: &PgConnection) -> &'static str {
        use super::schema::posts::dsl::*;
        diesel::delete(posts.find(_id)).execute(_conn).is_ok();

        "Post has been deleted"
    }
    
}

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