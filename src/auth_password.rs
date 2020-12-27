use serde::{Deserialize};
use rocket::request::{FromForm};

use crate::auth::{UserAuthenticator, AuthenticationResult};
use crate::db_sqlite::{DbConn, fetch_user_by_email};
use crate::auth::AuthenticationResult::AuthenticatedUser;


#[derive(FromForm, Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

impl UserAuthenticator for LoginInfo {
    fn authenticate(&self, db: &DbConn) -> AuthenticationResult {
        let maybe_user = fetch_user_by_email(db, &self.email);
        match maybe_user {
            Some(user) => {
                let hash = hash_password(&self.password);
                if Some(hash) == user.password_hash {
                    return AuthenticatedUser(user);
                } else {
                    AuthenticationResult::FailedWithEmail(self.email.clone())
                }
            }
            None => AuthenticationResult::Error("Error during authentication".to_string())
        }
    }
}

// helper functions
pub fn hash_password(password: &String) -> String {
    // let mut hasher = Sha3::sha3_256();
    // hasher.input_str(password);
    // hasher.result_str()
    password.clone()
}
