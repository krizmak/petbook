use chrono::{NaiveDate, Utc};
use rocket::request::FromFormValue;
use rocket::http::RawStr;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct NaiveDateForm(NaiveDate);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        let naive_date = NaiveDate::parse_from_str(form_value, "%Y-%m-%d").map_err(|x| form_value)?;
        Ok(NaiveDateForm(naive_date))
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDate;

    fn deref(&self)->&NaiveDate{
        &self.0
    }
}