use rocket::request::{FromForm};
use serde::{Serialize};
use chrono::NaiveDate;
use crate::schema::dogs;
use crate::dog::forms::NaiveDateForm;

#[derive(FromForm,Debug,Clone)]
pub struct DogForm {
    pub name : String,
    pub breed : i32,
    pub sex : String,
    pub color : i32,
    pub chip_id : Option<String>,
    pub description : Option<String>,
    pub birth : NaiveDateForm,
    pub death : Option<NaiveDateForm>,
}

#[derive(Insertable,Debug,Clone)]
#[table_name="dogs"]
pub struct Dog {
    pub name : String,
    pub breed : i32,
    pub sex : String,
    pub color : i32,
    pub chip_id : Option<String>,
    pub description : Option<String>,
    pub birth : NaiveDate,
    pub death : Option<NaiveDate>,
    pub owner_id : i32,
    pub address_id : Option<i32>,
}

#[derive(Queryable,Debug,Clone,Serialize)]
pub struct DogEntity {
    pub id : i32,
    pub name : String,
    pub breed : i32,
    pub sex : String,
    pub color : i32,
    pub chip_id : Option<String>,
    pub description : Option<String>,
    pub birth : NaiveDate,
    pub death : Option<NaiveDate>,
    pub owner_id : i32,
    pub address_id : Option<i32>,
}

