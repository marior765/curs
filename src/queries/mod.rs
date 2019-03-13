#[path="../schema.rs"]
mod schema;
#[path="../models.rs"]
mod models;

// use diesel::prelude::*;
// use schema::*;
// use models::{InsertablePerson, Person};

// pub fn all(conn: &PgConnection) -> QueryResult<Vec<Person>> {
//     people::table.load::<Person>(&*conn)
// }

// // pub fn get(id: i32, conn: &PgConnection) -> QueryResult<Person> {
// //     people::table.find(id).get_result::<Person>(conn)
// // } 

// pub fn insert(person: Person, conn: &PgConnection) -> QueryResult<Person> {
//     diesel::insert_into(people::table)
//         .values(&InsertablePerson::from_person(person))
//         .get_result(conn)
// }

// pub fn update(id: i32, person: Person, conn: &PgConnection) -> QueryResult<Person> {
//     diesel::update(people::table.find(id))
//         .set(&person)
//         .get_result(conn)
// }
// pub fn delete(id: i32, conn: &PgConnection) -> QueryResult<usize> {
//     diesel::delete(people::table.find(id))
//         .execute(conn)
// }