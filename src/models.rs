// #[path="./schema.rs"]
// mod schema;

// // use schema::*;
// // use diesel::prelude::*;
// // use serde::prelude::*;

// // #[derive(Queryable, Serialize, Deserialize)]
// pub struct Post {
//     pub id: i32,  
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }

// // #[derive(Insertable, Identifiable, Queryable)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

// #[derive(Queryable)]
// pub struct Person {
//     pub id: i32,
//     pub first_name: String,
//     pub last_name: String,
//     pub age: i32,
//     pub profession: String,
//     pub salary: i32,
// }

// // #[derive(Insertable)]
// pub struct InsertablePerson {
//     first_name: String,
//     last_name: String,
//     age: i32,
//     profession: String,
//     salary: i32,
// }

// impl InsertablePerson {

//     pub fn from_people(person: Person) -> InsertablePerson {
//         InsertablePerson {
//             first_name: person.first_name,
//             last_name: person.last_name,
//             age: person.age,
//             profession: person.profession,
//             salary: person.salary,
//         }
//     }
// }
