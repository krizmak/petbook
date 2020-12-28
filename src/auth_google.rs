use serde::{Deserialize};
use rocket::request::{FromForm};
use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey, decode_header};
use std::collections::HashMap;
use std::time::Duration;
use crate::auth::{UserAuthenticator, AuthenticationResult, UserCreator, UserCreationResult};
use crate::db_sqlite::DbConn;
use crate::models::User;


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

fn fetch_keys() -> reqwest::Result<JwkKeys> {
    let http_response = reqwest::blocking::get("https://www.googleapis.com/oauth2/v3/certs")?;
    let max_age = FALLBACK_TIMEOUT;
    let result = http_response.json::<KeyResponse>()?;

    return reqwest::Result::Ok(
        JwkKeys {
            keys: keys_to_map(result.keys),
            validity: max_age,
        });
}

fn keys_to_map(keys: Vec<JwkKey>) -> HashMap<String, JwkKey> {
    let mut keys_as_map = HashMap::new();
    for key in keys {
        keys_as_map.insert(String::clone(&key.kid), key);
    }
    keys_as_map
}

pub fn decode_token(token: &String) -> Claims {
    let header = decode_header(&token).expect("header error");
    println!("{:?}",header);

    let jwkkeys = fetch_keys().expect("key_fetch error");
    println!("google keys: {:?}", jwkkeys);

    let kid = match header.kid {
        Some(kid) => kid,
        None => panic!("kid error")
    };

    let key = jwkkeys.keys.get(&kid).expect("key id error");
    println!("google keys: {:?}", key);

    let maybe_decoded_token = decode::<Claims>(&token,
                                               &DecodingKey::from_rsa_components(&key.n, &key.e),
                                               &Validation::new(Algorithm::RS256));

    let decoded_token = match maybe_decoded_token {
        Ok(decoded_token) => decoded_token,
        Err(error) => panic!("Problem with decoding {:?}",error),
    };

    println!("{:?}",decoded_token.claims.sub);
    return decoded_token.claims;
}

#[derive(FromForm, Deserialize)]
pub struct GoogleLoginInfo {
    pub idtoken: String
}

impl UserAuthenticator for GoogleLoginInfo {
    fn authenticate(&self, db: &DbConn) -> AuthenticationResult {
        let claims = decode_token(&self.idtoken);
        println!("Fetching user by google_id: {}", &claims.sub);
        let maybe_user = db.get_user_by_google_id(&claims.sub);
        match maybe_user {
            Some(user) => {
                println!("Found user by google_id: {}", &user.email );
                AuthenticationResult::AuthenticatedUser(user)
            },
            None => AuthenticationResult::FailedWithEmail(claims.email.clone())
        }
    }
}

#[derive(FromForm, Deserialize)]
pub struct GoogleCreateInfo {
    pub name: String,
    pub email: String,
    pub idtoken: String,
}

impl UserCreator for GoogleCreateInfo {
    fn create(&self, db: &DbConn) -> UserCreationResult {
        let google_user_data = decode_token(&self.idtoken);
        let user = User {
            name: self.name.clone(),
            email: self.email.clone(),
            age: None,
            password_hash: None,
            google_id: Some(google_user_data.sub.clone()),
            facebook_id: None,
        };
        return UserCreationResult::User(db.insert_user(&user));
    }
}
