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
    Checkbox{name: String, values: Vec<(String, String)>, checked_value: String, label: String},
    Select{name: String, values: Vec<(String, String)>, selected_value: String, label: String},
    Text{name: String, value: String, label: String},
    Date{name: String, value: String, label: String},
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
                FormInputType::Text {name, value, label} => {
                    ret.push_str(&format!(r#"
                        <div class="mb-3">
                          <label for="{n}" class="form-label">{l}</label>
                          <input type="text" class="form-control" id="{n}" name="{n}" aria-describedby="{n}Help" value="{v}"/>
                        </div>"#, n=name, l=label, v=value));
                },
                FormInputType::Date {name, value, label} => {
                    ret.push_str(&format!(r#"
                        <div class="mb-3">
                          <label for="{n}" class="form-label">{l}</label>
                          <input type="date" class="form-control" id="{n}" name="{n}" aria-describedby="{n}Help" value="{v}"/>
                        </div>"#, n=name, l=label, v=value));
                },
                FormInputType::Select {name, values,selected_value, label} => {
                    ret.push_str(&format!(r#"
                        <div class="mb-3">
                          <label for="{n}" class="form-label">{l}</label>
                          <select class="form-select" aria-label="Default select example" id="{n}" name="{n}">"#, n=name, l=label));
                    for (value, label) in values {
                        let selected:String;
                        if selected_value == value {
                            selected = "selected".to_owned();
                        } else {
                            selected = "".to_owned();
                        }
                        ret.push_str(&format!(r#"
                            <option value="{v}" {s}>{l}</option>"#, v=value, l=label, s=selected ));
                    };
                    ret.push_str("
                        </select>
                        </div>");
                },
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
        let my_form = HtmlForm{ fields: vec![
            FormInputType::Text {
                name : "elso".to_owned(),
                value : "talán".to_owned(),
                label : "label_elso".to_owned()},
            FormInputType::Select {
                name : "select_mező".to_owned(),
                values : vec![("egy".to_owned(),"megérett a meggy".to_owned()), ("kettő".to_owned(),"csipkebokor vessző".to_owned())],
                selected_value : "egy".to_owned(),
                label : "label_select_mező".to_owned(),
            }
        ] };
        println!("{}",&my_form.to_html());
        assert_eq!("elso", "elso");
    }

}