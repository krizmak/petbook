/*
    COLOR,
    DATE,
    DATETIME_LOCAL,
    EMAIL,
    FILE,
    HIDDEN,
    IMAGE,
    MONTH,
    NUMBER,
    PASSWORD,
    RADIO,
    RANGE,
    SEARCH,
    TEL,
    TIME,
    URL,
    WEEK

 */

pub enum FormInputType {
    Checkbox{label :String, id:String, name:String, value:String, checked: bool},
    Text{label :String, id:String, name:String, value:String},
}

/*
pub struct FormInput {
    pub label: String,
    pub id: String,
    pub name: String,
    pub value: String,
    pub checked: Option<bool>,
    pub disabled: Option<bool>,
    pub max: Option<u32>,
    pub max_length: Option<u32>,
    pub min: Option<u32>,
    pub pattern: Option<String>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<u32>,
    pub step: Option<u32>,
}
*/

pub struct HtmlForm {
    pub fields : Vec<FormInputType>,
}

impl HtmlForm {
    pub fn to_html(&self) -> String {
        let mut ret = "".to_owned();
        for field in &self.fields {
            match field {
                FormInputType::Text { label, id, name, value } => { ret.push_str(name) },
                _ => {}
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let my_form = HtmlForm{ fields: vec![FormInputType::Text {
            label: "".to_string(),
            id: "".to_string(),
            name: "elso".to_string(),
            value: "".to_string(),
        }] };
        let value = 10;
        assert_eq!("elso", my_form.to_html());
    }

}