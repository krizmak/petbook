use serde::{Deserialize};
use rocket::request::{FromForm};

#[derive(FromForm, Deserialize)]
pub struct UserCreateInfo {
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub password: String
}


#[derive(FromForm, Deserialize)]
pub struct GoogleCreateInfo {
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub idtoken: String,
}

#[derive(FromForm, Deserialize)]
pub struct FacebookCreateInfo {
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub idtoken: String,
}
