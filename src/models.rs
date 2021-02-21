use rocket::request::{FromForm};

use serde::{Serialize};
use crate::schema::{addresses, users};

//Address
#[derive(Insertable,Debug,Serialize,Clone)]
#[table_name="addresses"]
pub struct Address {
    pub country : String,
    pub state : Option<String>,
    pub county : Option<String>,
    pub city : Option<String>,
    pub postal_code : Option<String>,
    pub street : Option<String>,
    pub address_line : Option<String>,
}

#[derive(Queryable,Debug,Clone,Serialize)]
pub struct AddressEntity {
    pub id : i32,
    pub state : Option<String>,
    pub county : Option<String>,
    pub city : Option<String>,
    pub postal_code : Option<String>,
    pub street : Option<String>,
    pub address_line : Option<String>,
}


// User
#[derive(Insertable,FromForm,Debug,Serialize,Clone)]
#[table_name="users"]
pub struct User {
    pub name : String,
    pub informal_name : Option<String>,
    pub title : Option<String>,
    pub email : String,
    pub address_id : Option<i32>,
    pub phone : Option<String>,
    pub password_hash : Option<String>,
    pub google_id : Option<String>,
    pub facebook_id : Option<String>,
    pub disabled : Option<bool>,
}

#[derive(Queryable,Debug,Clone,Serialize)]
pub struct UserEntity {
    pub id: i32,
    pub name : String,
    pub informal_name : Option<String>,
    pub title : Option<String>,
    pub email : String,
    pub address_id : Option<i32>,
    pub phone : Option<String>,
    pub password_hash : Option<String>,
    pub google_id : Option<String>,
    pub facebook_id : Option<String>,
    pub disabled : Option<bool>,
}

#[derive(Debug)]
pub enum UserAuthorizationError {
    NoUserFound,
    GoogleError,
    FacebookError,
}
