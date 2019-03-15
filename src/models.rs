#[path="./schema.rs"]
mod schema;

use diesel::{Queryable, Insertable, Identifiable};
use serde::{Serialize, Deserialize};

use schema::{posts, people};

#[derive(Queryable, Serialize, Deserialize)]
#[column_name="posts"]
pub struct Post {
    pub id: i32,  
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Queryable)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Identifiable)]
#[column_name="people"]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub profession: String,
    pub salary: i32,
}

#[derive(Insertable)]
pub struct InsertablePerson {
    first_name: String,
    last_name: String,
    age: i32,
    profession: String,
    salary: i32,
}

impl InsertablePerson {

    pub fn from_people(person: Person) -> InsertablePerson {
        InsertablePerson {
            first_name: person.first_name,
            last_name: person.last_name,
            age: person.age,
            profession: person.profession,
            salary: person.salary,
        }
    }
}
