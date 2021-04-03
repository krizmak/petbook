use rocket::request::{FromForm, Form};
use serde::{Serialize};
use chrono::NaiveDate;
use diesel::query_builder::AsChangeset;
use crate::schema::dogs;
use crate::dog::forms::NaiveDateForm;
use crate::widget::widgets::{HtmlForm, FormInputType};
use crate::db_sqlite::DbConn;
use std::collections::HashMap;
use diesel::QueryResult;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::UserEntity;

macro_rules! build_model {
    ( $struct_name:ident; $entity_struct_name:ident; $table_id:ident; $table:expr => {
        $( $attr_name:ident : $attr_type:ty ),*
    }) => {
        #[table_name=$table]
        #[derive(Serialize, Insertable, Debug, Clone)]
        pub struct $struct_name {
            $( pub $attr_name : $attr_type ),*
        }

        #[table_name=$table]
        #[derive(Queryable, Serialize, Debug, Clone, AsChangeset)]
        pub struct $entity_struct_name {
            pub id: i32,
            $( pub $attr_name : $attr_type ),*
        }

        impl $struct_name {
            pub fn from_entity(entity: $entity_struct_name) -> (i32, $struct_name) {
                (entity.id,
                 $struct_name {
                    $( $attr_name : entity.$attr_name ),*
                 })
            }

        }

        impl $entity_struct_name {
            pub fn from(entity_id: i32, base_struct: &$struct_name) -> $entity_struct_name {
                $entity_struct_name {
                    id : entity_id,
                    $( $attr_name : base_struct.$attr_name.clone() ),*
                }
            }
        }

       impl $struct_name {
            pub fn get(get_id: i32, db: &DbConn) -> QueryResult<(i32, $struct_name)> {
                use crate::schema::$table_id::dsl::*;

                let entity = $table_id
                    .filter(id.eq(get_id))
                    .first::<$entity_struct_name>(&db.0)?;
                Ok($struct_name::from_entity(entity))
            }

            pub fn insert(&self, db: &DbConn) -> QueryResult<(i32,$struct_name)> {
                use crate::schema::$table_id::dsl::*;
                use crate::schema::$table_id::dsl::id;

                diesel::insert_into($table_id)
                    .values(self)
                    .execute(&db.0)?;

                let entity: $entity_struct_name = $table_id
                    .order(id.desc())
                    .first(&db.0)?;

                Ok($struct_name::from_entity(entity))
            }
       }

       impl $struct_name {
            pub fn update( update_id: i32, new_values: &$struct_name,db: &DbConn) -> QueryResult<(i32, $struct_name)>
            {
                use crate::schema::$table_id::dsl::*;
                use crate::schema::$table_id::dsl::id;

                let new_entity = $entity_struct_name::from(update_id, new_values);
                print!("{:?}", &new_entity);
                diesel::update($table_id.filter(id.eq(update_id)))
                    .set(&new_entity)
                    .execute(&db.0)?;

                let updated_entity : $entity_struct_name =
                    $table_id.filter(id.eq(update_id))
                    .first(&db.0)?;

                Ok($struct_name::from_entity(updated_entity))
            }
       }
    }
}

build_model!(Dog; DogEntity; dogs; "dogs" => {
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

pub struct DogHtmlForm(HtmlForm);

impl DogForm {
    pub fn from_dog(dog : Option<&Dog>, db : &DbConn) -> String {
        let dog_breeds= db.fetch_dog_breeds().expect("db_error");
        let breeds = dog_breeds.iter().map(|x| (x.id.to_string(), x.name.to_owned() )).collect::<Vec<(_,_)>>();
        let sexes = vec![("m".to_owned(),"male".to_owned()),("f".to_owned(),"female".to_owned())];
        let colors = vec![
            (1.to_string(),"white".to_owned()),
            (2.to_string(),"brownish".to_owned()),
            (3.to_string(),"brown".to_owned()),
            (4.to_string(),"dark brown".to_owned()),
        ];
        let default_dog = Dog {
            name: "".to_string(),
            breed: 0,
            sex: "".to_string(),
            color: 0,
            chip_id: None,
            description: None,
            birth: NaiveDate::from_ymd(2015, 3, 14),
            death: None,
            owner_id: 0,
            address_id: None
        };
        let d = dog.unwrap_or(&default_dog);

        let fields: Vec<FormInputType> = vec![
            FormInputType::Text{name: "name".to_owned(), value: d.name.to_owned(), label : "Name".to_owned()},
            FormInputType::Select{name: "breed".to_owned(), values: breeds, selected_value: d.breed.to_string(), label: "Breed".to_owned() },
            FormInputType::Select{name: "sex".to_owned(), values: sexes, selected_value: d.sex.to_owned(), label: "Sex".to_owned() },
            FormInputType::Select{name: "color".to_owned(), values: colors, selected_value: d.color.to_string(), label: "Color".to_owned() },
            FormInputType::Text{name: "chip_id".to_owned(), value: d.chip_id.as_ref().map_or("".to_owned(), |x| x.to_string()), label : "Chip ID".to_owned()},
            FormInputType::Date{name: "birth".to_owned(), value: d.birth.to_string(), label : "Birth".to_owned()},
            FormInputType::Date{name: "death".to_owned(), value: d.death.map_or("".to_owned(),|x| x.to_string()), label : "Death".to_owned()},
        ];
        HtmlForm{fields}.to_html()
    }

    pub fn to_dog(&self, owner: &UserEntity) -> Dog {
        Dog{
            name : self.name.to_string(),
            breed : self.breed,
            sex : self.sex.to_string(),
            color : self.color,
            chip_id : self.chip_id.to_owned(),
            description : self.description.to_owned(),
            birth : *self.birth.to_owned(),
            death : self.death.as_ref().map(|x| **x),
            owner_id: owner.id,
            address_id : None
        }
    }
}


#[derive(Queryable,Debug,Clone,Serialize)]
pub struct DogBreedEntity {
    pub id: i32,
    pub name: String
}