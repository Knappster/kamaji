use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Config {
    pub name: String,
    pub value: String,
}
