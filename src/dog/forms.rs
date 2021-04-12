use rocket::request::FromForm;
use crate::util::forms::NaiveDateForm;
use crate::dog::models::{Dog, Log};
use crate::db_sqlite::DbConn;
use chrono::{NaiveDate, Utc};
use crate::widget::widgets::{FormInputType, HtmlForm};
use crate::user::models::UserEntity;

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

#[derive(FromForm,Debug,Clone)]
pub struct LogForm {
    pub log_date : NaiveDateForm,
    pub summary : String,
    pub description : Option<String>,
}

impl LogForm {
    pub fn from_object(log: Option<&Log>) -> String
    {
        let default_log = Log {
            log_date: Utc::today().naive_local(),
            summary: "".to_string(),
            description: None,
            dog_id: 0
        };
        let l = log.unwrap_or(&default_log);

        let fields: Vec<FormInputType> = vec![
            FormInputType::Date{name: "log_date".to_owned(), value: l.log_date.to_string(), label : "Date".to_owned()},
            FormInputType::Text{name: "summary".to_owned(), value: l.summary.to_owned(), label : "Summary".to_owned()},
            FormInputType::Text{name: "description".to_owned(), value: l.description.as_ref().map_or("".to_owned(), |x| x.to_string()), label : "Description".to_owned()},
        ];
        HtmlForm{fields}.to_html()
    }

    pub fn to_object(&self, log: &mut Log) {
        log.log_date = *self.log_date;
        log.summary = self.summary.to_owned();
        log.description = self.description.to_owned();
    }

}