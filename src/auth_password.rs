use serde::{Deserialize};
use rocket::request::{FromForm};

use crate::auth::{UserAuthenticator, UserCreator, AuthenticationError, UserCreationError};
use crate::db_sqlite::DbConn;
use crate::models::{User, UserEntity};


#[derive(FromForm, Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

impl UserAuthenticator for LoginInfo {
    fn authenticate(&self, db: &DbConn) -> Result<UserEntity, AuthenticationError> {
        let user = db.get_user_by_email(&self.email)
            .map_err(|err| match err {
                diesel::NotFound => AuthenticationError::Failed,
                _ => AuthenticationError::InternalError("Database error".to_string())
            })?;
        let user_hash = user.password_hash.clone().ok_or_else(|| AuthenticationError::Failed)?;
        if hash_password(&self.password) == user_hash {
            Ok(user)
        } else {
            Err(AuthenticationError::Failed)
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
    fn create(&self, db: &DbConn) -> Result<UserEntity, UserCreationError> {
        let user = User {
            name: self.name.clone(),
            informal_name : None,
            title: None,
            email: self.email.clone(),
            address_id: None,
            phone: None,
            password_hash: Some(hash_password(&self.password)),
            google_id: None,
            facebook_id: None,
            disabled: None
        };
        db.insert_user(&user)
            .map_err(|err| match err {
                diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) =>
                    UserCreationError::FailedAlreadyExists,
                _ => UserCreationError::InternalError("Database error".to_string())
            })
    }
}


// helper functions
pub fn hash_password(password: &String) -> String {
    // let mut hasher = Sha3::sha3_256();
    // hasher.input_str(password);
    // hasher.result_str()
    password.clone()
}
