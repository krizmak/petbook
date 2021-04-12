use rocket::request::{FromForm};
use crate::widget::widgets::{HtmlForm, FormInputType};
use super::models::{User,Address};

#[derive(FromForm,Debug,Clone)]
pub struct UserForm {
    pub name : String,
    pub informal_name : Option<String>,
    pub title : Option<String>,
    pub email : String,
    pub phone : Option<String>,

    pub country : String,
    pub state : Option<String>,
    pub county : Option<String>,
    pub city : Option<String>,
    pub postal_code : Option<String>,
    pub street : Option<String>,
    pub address_line : Option<String>
}

impl UserForm {
    pub fn from_objects(user : &User, address: &Address) -> String {
        let fields: Vec<FormInputType> = vec![
            FormInputType::Text{name: "name".to_owned(), value: user.name.to_owned(), label : "Name".to_owned()},
            FormInputType::Text{name: "informal_name".to_owned(), value: user.informal_name.as_ref().map_or("".to_owned(), |x| x.to_owned()), label: "Informal name".to_owned() },
            FormInputType::Text{name: "title".to_owned(), value: user.title.as_ref().map_or("".to_owned(), |x| x.to_owned()), label: "Title".to_owned() },
            FormInputType::Text{name: "email".to_owned(), value: user.email.to_owned(), label : "Email".to_owned()},
            FormInputType::Text{name: "phone".to_owned(), value: user.phone.as_ref().map_or("".to_owned(), |x| x.to_owned()), label : "Phone".to_owned()},
            FormInputType::Text{name: "country".to_owned(), value: address.country.to_owned(), label : "Country".to_owned()},
            FormInputType::Text{name: "state".to_owned(), value: address.state.as_ref().map_or("".to_owned(), |x| x.to_owned()), label : "State".to_owned()},
            FormInputType::Text{name: "county".to_owned(), value: address.county.as_ref().map_or("".to_owned(), |x| x.to_owned()), label : "County".to_owned()},
            FormInputType::Text{name: "city".to_owned(), value: address.city.as_ref().map_or("".to_owned(), |x| x.to_owned()), label : "City".to_owned()},
            FormInputType::Text{name: "postal_code".to_owned(), value: address.postal_code.as_ref().map_or("".to_owned(), |x| x.to_owned()), label : "Postal code".to_owned()},
            FormInputType::Text{name: "street".to_owned(), value: address.street.as_ref().map_or("".to_owned(), |x| x.to_owned()), label : "Street".to_owned()},
            FormInputType::Text{name: "address_line".to_owned(), value: address.address_line.as_ref().map_or("".to_owned(), |x| x.to_owned()), label : "Address line".to_owned()},
        ];
        HtmlForm{fields}.to_html()
    }

    pub fn to_objects(&self, user: &mut User, address: &mut Address) {
        user.name = self.name.to_owned();
        user.informal_name = self.informal_name.to_owned();
        user.title = self.title.to_owned();
        user.email = self.email.to_owned();
        user.phone = self.phone.as_ref().map(|x| x.to_owned());
        address.country = self.country.to_owned();
        address.state = self.state.to_owned();
        address.county = self.county.to_owned();
        address.city = self.city.to_owned();
        address.postal_code = self.postal_code.to_owned();
        address.street = self.street.to_owned();
        address.address_line = self.address_line.to_owned();
    }
}

