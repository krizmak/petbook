use serde::Serialize;
use chrono::{NaiveDate, Utc};
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

impl Log {
    pub fn get_pet_logs(pet_id: &i32, db: &DbConn) -> Vec<(i32, Log)> {
        use crate::schema::logs::dsl::*;
        let log_entities: Vec<LogEntity> = logs
            .filter(dog_id.eq(pet_id))
            .load::<LogEntity>(&db.0).unwrap();
        log_entities
            .iter()
            .map(|entity| Log::from_entity(entity.to_owned()))
            .collect()
    }

    pub fn new(maybe_dog_id: Option<i32>) -> Log {
        Log {
            log_date: Utc::today().naive_local(),
            summary: "".to_string(),
            description: None,
            dog_id: maybe_dog_id.unwrap_or_else(|| -1)
        }
    }
}