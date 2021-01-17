use serde::{Deserialize};
use rocket::request::{FromForm};
use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey, decode_header};
use std::collections::HashMap;
use std::time::Duration;
use crate::auth::{UserAuthenticator, AuthenticationError, UserCreator, UserCreationError};
use crate::db_sqlite::DbConn;
use crate::models::{User, UserEntity};


#[derive(Debug, Deserialize)]
pub struct Claims {
    pub iss: String,          // The token issuer
    pub sub: String,          // The subject the token refers to
    pub aud: String,          // The audience the token was issued for
    pub iat: i64,             // Issued at -- as epoch seconds
    pub exp: i64,             // The expiry date -- as epoch seconds
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub locale: String,
    pub picture: String,
}


#[derive(Debug, Deserialize, Eq, PartialEq)]
struct JwkKey {
    e: String,
    alg: String,
    kty: String,
    kid: String,
    n: String,
}

#[derive(Debug)]
struct JwkKeys {
    keys: HashMap<String, JwkKey>,
    validity: Duration,
}

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<JwkKey>,
}

const FALLBACK_TIMEOUT: Duration = Duration::from_secs(60);

pub enum GoogleErrors {
    HeaderParseError,
    KeyFetchError,
    DecodeError
}

fn fetch_keys() -> reqwest::Result<JwkKeys> {
    let http_response = reqwest::blocking::get("https://www.googleapis.com/oauth2/v3/certs")?;
    let max_age = FALLBACK_TIMEOUT;
    http_response.json::<KeyResponse>()
        .map(|result| JwkKeys {
            keys: keys_to_map(result.keys),
            validity: max_age,
        })
}

fn keys_to_map(keys: Vec<JwkKey>) -> HashMap<String, JwkKey> {
    let mut keys_as_map = HashMap::new();
    for key in keys {
        keys_as_map.insert(String::clone(&key.kid), key);
    }
    keys_as_map
}

pub fn decode_token(token: &String) -> Result<Claims, GoogleErrors> {
    let header = decode_header(&token).map_err(|_| GoogleErrors::HeaderParseError)?;
    let kid = header.kid.ok_or_else(|| GoogleErrors::HeaderParseError)?;

    let jwkkeys = fetch_keys().map_err(|_| GoogleErrors::KeyFetchError)?;
    let key = jwkkeys.keys.get(&kid).ok_or_else(|| GoogleErrors::KeyFetchError)?;

    let decoded_token =
        decode::<Claims>(&token,
                         &DecodingKey::from_rsa_components(&key.n, &key.e),
                         &Validation::new(Algorithm::RS256))
            .map_err(|_| GoogleErrors::DecodeError)?;

    Ok(decoded_token.claims)
}

#[derive(FromForm, Deserialize)]
pub struct GoogleLoginInfo {
    pub idtoken: String
}

impl UserAuthenticator for GoogleLoginInfo {
    fn authenticate(&self, db: &DbConn) -> Result<UserEntity, AuthenticationError> {
        let claims = decode_token(&self.idtoken)
            .map_err(|_| AuthenticationError::InternalError("Google internal error".to_string()))?;

        db.get_user_by_google_id(&claims.sub)
            .map_err(|err| match err {
                diesel::NotFound => AuthenticationError::FailedWithEmail(claims.email.clone()),
                _ => AuthenticationError::InternalError("Database error".to_string())
            })
    }
}

#[derive(FromForm, Deserialize)]
pub struct GoogleCreateInfo {
    pub name: String,
    pub email: String,
    pub idtoken: String,
}

impl UserCreator for GoogleCreateInfo {
    fn create(&self, db: &DbConn) -> Result<UserEntity, UserCreationError> {
        let google_user_data = decode_token(&self.idtoken)
            .map_err(|_| UserCreationError::InternalError("Google error".to_owned()))?;
        let user = User {
            name: self.name.clone(),
            informal_name: None,
            title: None,
            email: self.email.clone(),
            address_id: None,
            phone: None,
            password_hash: None,
            google_id: Some(google_user_data.sub.clone()),
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
