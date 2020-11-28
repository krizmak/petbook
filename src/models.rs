use rocket::request::FromForm;
use serde::{Serialize};
use crate::schema::users;
use crate::schema::userauth;

// User
#[derive(Insertable,FromForm)]
#[table_name="users"]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Queryable,Debug,Clone,Serialize)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32
}

// UserAuth
#[derive(Insertable)]
#[table_name="userauth"]
pub struct UserAuth {
    pub user_id: i32,
    pub password_hash: String
}

#[derive(Queryable,Debug,Clone,Serialize)]
pub struct UserAuthEntity {
    pub id: i32,
    pub user_id: i32,
    pub password_hash: String
}

