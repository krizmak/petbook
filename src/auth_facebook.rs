use serde::{Deserialize};
use rocket::request::{FromForm};

use crate::auth::{UserAuthenticator, AuthenticationResult, UserCreator, UserCreationResult};
use crate::db_sqlite::DbConn;
use crate::auth::AuthenticationResult::AuthenticatedUser;
use crate::models::User;

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

pub fn decode_token(user_token: &str) -> FbUserData {
    let client_id = "1925360770951525";
    let client_secret = "2d76326a95b256fb7118cb772e72e71c";
    let app_url = "https://graph.facebook.com/oauth/access_token?client_id=".to_owned()
        + &client_id
        + "&client_secret="
        + &client_secret
        + "&grant_type=client_credentials";

    let http_response = reqwest::blocking::get(&app_url).expect("app http request error");
    let result = http_response.json::<FbAppToken>().expect("app http response error");
    let app_token = result.access_token;

    let user_url = "https://graph.facebook.com/debug_token?input_token=".to_owned()
        +user_token
        +"&access_token="
        +&app_token;
    let http_response = reqwest::blocking::get(&user_url).expect("app http request error");

    let result = http_response.json::<Claims>().expect("app http response error");
    println!("{:?}", result);


    let profile_url = format!("https://graph.facebook.com/v9.0/{}?access_token={}&fields=name,email", result.data.user_id, user_token);
    let http_response = reqwest::blocking::get(&profile_url).expect("profile http request error");

    let user_data = http_response.json::<FbUserData>().expect("profile http response error");

    return user_data;
}

#[derive(FromForm, Deserialize)]
pub struct FacebookLoginInfo {
    pub idtoken: String
}

impl UserAuthenticator for FacebookLoginInfo {
    fn authenticate(&self, db: &DbConn) -> AuthenticationResult {
        let user_data = decode_token(&self.idtoken);
        let maybe_user = db.get_user_by_facebook_id(&user_data.id);
        match maybe_user {
            Some(user) => AuthenticatedUser(user),
            None => AuthenticationResult::FailedWithEmail(user_data.email.clone())
        }
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
    fn create(&self, db: &DbConn) -> UserCreationResult {
        let facebook_user_data = decode_token(&self.idtoken);
        let user = User {
            name: self.name.clone(),
            email: self.email.clone(),
            age: None,
            password_hash: None,
            google_id: None,
            facebook_id: Some(facebook_user_data.id.clone()),
        };
        return UserCreationResult::User(db.insert_user(&user));
    }
}
