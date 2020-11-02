use rocket::request::FromForm;
use serde::{Serialize};
use crate::schema::users;


#[derive(Queryable,Debug,Clone,Serialize)]
pub struct UserEntity {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub age: i32
}

#[derive(Insertable,FromForm)]
#[table_name="users"]
pub struct User {
  pub name: String,
  pub email: String,
  pub age: i32
}

