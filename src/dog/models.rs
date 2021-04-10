
use rocket::request::{FromForm};
use serde::{Serialize};
use chrono::NaiveDate;
use crate::schema::dogs;
use crate::util::forms::NaiveDateForm;
use crate::widget::widgets::{HtmlForm, FormInputType};
use crate::db_sqlite::DbConn;
use diesel::QueryResult;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::user::models::UserEntity;

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