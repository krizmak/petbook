use serde::{Deserialize};
use rocket::request::{FromForm};

use crate::auth::{UserAuthenticator, AuthenticationResult, UserCreator, UserCreationResult};
use crate::db_sqlite::{DbConn, fetch_user_by_email, create_user};
use crate::auth::AuthenticationResult::AuthenticatedUser;
use crate::models::User;


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

#[derive(FromForm, Deserialize)]
pub struct UserCreateInfo {
    pub name: String,
    pub email: String,
    pub password: String
}

impl UserCreator for UserCreateInfo {
    fn create(&self, db: &DbConn) -> UserCreationResult {
        let user = User {
            name: self.name.clone(),
            email: self.email.clone(),
            age: None,
            password_hash: Some(hash_password(&self.password)),
            google_id: None,
            facebook_id: None,
        };
        return UserCreationResult::User(create_user(&db, &user));
    }
}


// helper functions
pub fn hash_password(password: &String) -> String {
    // let mut hasher = Sha3::sha3_256();
    // hasher.input_str(password);
    // hasher.result_str()
    password.clone()
}
