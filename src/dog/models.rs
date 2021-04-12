use serde::Serialize;
use chrono::NaiveDate;
use crate::schema::dogs;
use crate::schema::logs;
use crate::db_sqlite::DbConn;
use diesel::QueryResult;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

crate::build_model!(Dog; DogEntity; dogs; "dogs" => {
    name : String,
    breed : i32,
    sex : String,
    color : i32,
    chip_id : Option<String>,
    description : Option<String>,
    birth : NaiveDate,
    death : Option<NaiveDate>,
    owner_id : i32,
    address_id : Option<i32>
});

crate::build_model!(Log; LogEntity; logs; "logs" => {
    log_date : NaiveDate,
    summary : String,
    description : Option<String>,
    dog_id : i32
});

#[derive(Queryable,Debug,Clone,Serialize)]
pub struct DogBreedEntity {
    pub id: i32,
    pub name: String
}