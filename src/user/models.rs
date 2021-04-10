use serde::Serialize;
use diesel::QueryResult;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use crate::schema::users;
use crate::schema::addresses;
use crate::db_sqlite::DbConn;

// User
crate::build_model!(User; UserEntity; users; "users" => {
    name : String,
    informal_name : Option<String>,
    title : Option<String>,
    email : String,
    address_id : Option<i32>,
    phone : Option<String>,
    password_hash : Option<String>,
    google_id : Option<String>,
    facebook_id : Option<String>,
    disabled : Option<bool>
});

//Address
crate::build_model!(Address; AddressEntity; addresses; "addresses" => {
    country : String,
    state : Option<String>,
    county : Option<String>,
    city : Option<String>,
    postal_code : Option<String>,
    street : Option<String>,
    address_line : Option<String>
});

impl User {
    pub fn get_address(&self, db: &DbConn) -> (Option<i32>, Address) {
        match self.address_id {
            Some(address_id) => {
                let (_, address) = Address::get(address_id, &db).unwrap();
                (Some(address_id), address)
            },
            None => (None, Address{
                country: "".to_string(),
                state: None,
                county: None,
                city: None,
                postal_code: None,
                street: None,
                address_line: None
            })
        }
    }
}