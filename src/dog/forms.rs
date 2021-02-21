use chrono::{NaiveDate, Utc};
use rocket::request::FromFormValue;
use rocket::http::RawStr;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct NaiveDateForm(NaiveDate);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        Ok(NaiveDateForm(Utc::today().naive_utc()))
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDate;

    fn deref(&self)->&NaiveDate{
        &self.0
    }
}