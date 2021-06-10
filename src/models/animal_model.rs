use crate::db;
use crate::schema::animals;
// use crate::schema::animals::dsl::*;
use diesel::prelude::*;

#[derive(Debug, Queryable, PartialEq)]
pub struct Animal {
    species: String,
    name: Option<String>,
    legs: i32,
}

impl Animal {
    fn new<S: Into<String>>(species: S, name: Option<&str>, legs: i32) -> Self {
        Animal {
            species: species.into(),
            name: name.map(Into::into),
            legs,
        }
    }
}
