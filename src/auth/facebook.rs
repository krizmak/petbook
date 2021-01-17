use serde::{Deserialize};
use rocket::request::{FromForm};

use crate::auth::{UserAuthenticator, UserCreator, AuthenticationError, UserCreationError};
use crate::db_sqlite::DbConn;
use crate::models::{User, UserEntity};

#[derive(Debug, Deserialize)]
struct FbAppToken {
    access_token: String,
    token_type: String
}

#[derive(Debug, Deserialize)]
struct Claims {
    data: FbData,
}

#[derive(Debug, Deserialize)]
struct FbData {
    app_id: String,
    application: String,
    expires_at: i64,
    is_valid: bool,
    user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FbUserData {
    pub name: String,
    pub email: String,
    pub id: String,
}

pub enum Error {
    AppTokenError,
    UserTokenError,
    UserProfileError,
}

pub fn decode_token(user_token: &str) -> Result<FbUserData, Error> {
    // 1. step: request app token
    let client_id = "1925360770951525";
    let client_secret = "2d76326a95b256fb7118cb772e72e71c";
    let app_url = "https://graph.facebook.com/oauth/access_token?client_id=".to_owned()
        + &client_id
        + "&client_secret="
        + &client_secret
        + "&grant_type=client_credentials";

    let http_response = reqwest::blocking::get(&app_url)
        .map_err(|_| Error::AppTokenError)?;
    let result = http_response.json::<FbAppToken>()
        .map_err(|_| Error::AppTokenError)?;
    let app_token = result.access_token;

    // 2. step: request user token
    let user_url = "https://graph.facebook.com/debug_token?input_token=".to_owned()
        +user_token
        +"&access_token="
        +&app_token;
    let http_response = reqwest::blocking::get(&user_url)
        .map_err(|_| Error::UserTokenError)?;
    let result = http_response.json::<Claims>()
        .map_err(|_| Error::UserTokenError)?;

    // 3. step: request user profile
    let profile_url = format!("https://graph.facebook.com/v9.0/{}?access_token={}&fields=name,email",
                              result.data.user_id,
                              user_token);
    let http_response = reqwest::blocking::get(&profile_url)
        .map_err(|_| Error::UserProfileError)?;

    http_response.json::<FbUserData>()
        .map_err(|_| Error::UserProfileError)

}

#[derive(FromForm, Deserialize)]
pub struct FacebookLoginInfo {
    pub idtoken: String
}

impl UserAuthenticator for FacebookLoginInfo {
    fn authenticate(&self, db: &DbConn) -> Result<UserEntity, AuthenticationError> {
        let user_data = decode_token(&self.idtoken)
            .map_err(|_| AuthenticationError::InternalError("Facebook internal error.".to_string()))?;

        db.get_user_by_facebook_id(&user_data.id)
            .map_err(|err| match err {
                diesel::NotFound => AuthenticationError::FailedWithEmail(user_data.email.clone()),
                _ => AuthenticationError::InternalError("Database error".to_string())
            })
    }
}

#[derive(FromForm, Deserialize)]
pub struct FacebookCreateInfo {
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub idtoken: String,
}

impl UserCreator for FacebookCreateInfo {
    fn create(&self, db: &DbConn) -> Result<UserEntity, UserCreationError> {
        let facebook_user_data = decode_token(&self.idtoken)
            .map_err(|_| UserCreationError::InternalError("Facebook error".to_owned()))?;
        let user = User {
            name: self.name.clone(),
            informal_name: None,
            title: None,
            email: self.email.clone(),
            address_id: None,
            phone: None,
            password_hash: None,
            google_id: None,
            facebook_id: Some(facebook_user_data.id.clone()),
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
