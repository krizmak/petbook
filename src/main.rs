#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::collections::HashMap;
use std::time::Duration;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;

use diesel::prelude::*;
use serde::{Deserialize};
use serde_json::{json};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, decode_header};
use reqwest::*;

// use hello_rust::schema::userauth;
// use hello_rust::schema::users;
use petbook::models::{User, UserEntity, UserAuth, UserAuthEntity};

// types

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[derive(FromForm, Deserialize)]
pub struct UserCreateInfo {
    pub name: String,
    pub email: String,
    pub age: i32,
    pub password: String
}

#[derive(FromForm, Deserialize)]
struct LoginInfo {
    email: String,
    password: String,
}

#[derive(FromForm, Deserialize)]
struct GoogleLoginInfo {
    idtoken: String
}

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
pub struct JwkKey {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

#[derive(Debug)]
pub struct JwkKeys {
    pub keys: HashMap<String, JwkKey>,
    pub validity: Duration,
}

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<JwkKey>,
}

#[derive(FromForm, Deserialize)]
struct FacebookLoginInfo {
    idtoken: String
}

#[derive(Debug, Deserialize)]
struct FbAppToken {
    access_token: String,
    token_type: String
}


// Ok("{\"data\":{
//           \"app_id\":\"1925360770951525\",
//           \"type\":\"USER\",
//           \"application\":\"KrizmaTest - Test1\",
//           \"data_access_expires_at\":1615058100,
//           \"expires_at\":1607288400,
//           \"is_valid\":true,
//           \"scopes\":[\"email\",\"public_profile\"],
//           \"user_id\":\"1172382226451995\"}}")

#[derive(Debug, Deserialize)]
struct FbClaims {
    data: FbData,
}

#[derive(Debug, Deserialize)]
pub struct FbData {
    pub app_id: String, 
    pub application: String, 
    pub expires_at: i64, 
    pub is_valid: bool, 
    pub user_id: String,
}

const FALLBACK_TIMEOUT: Duration = Duration::from_secs(60);

pub fn fetch_keys() -> Result<JwkKeys> {
    let http_response = reqwest::blocking::get("https://www.googleapis.com/oauth2/v3/certs")?;
    let max_age = FALLBACK_TIMEOUT;
    let result = http_response.json::<KeyResponse>()?;

    return Result::Ok(
        JwkKeys {
            keys: keys_to_map(result.keys),
            validity: max_age,
        });
}

pub fn fb_validate_token(user_token: &str) -> Result<()> {
    let client_id = "1925360770951525";
    let client_secret = "2d76326a95b256fb7118cb772e72e71c";
    let app_url = "https://graph.facebook.com/oauth/access_token?client_id=".to_owned()
        + &client_id
        + "&client_secret="
        + &client_secret
        + "&grant_type=client_credentials";
    
    let http_response = reqwest::blocking::get(&app_url)?;
    let result = http_response.json::<FbAppToken>()?;
    let app_token = result.access_token;
    
    let user_url = "https://graph.facebook.com/debug_token?input_token=".to_owned()
        +user_token
        +"&access_token="
        +&app_token;
    let http_response = reqwest::blocking::get(&user_url)?;

    let result = http_response.json::<FbClaims>()?;
    println!("{:?}", result);


    let profile_url = format!("https://graph.facebook.com/v9.0/{}?access_token={}&fields=name,email", result.data.user_id, user_token);
    let http_response = reqwest::blocking::get(&profile_url)?;
    println!("{:?}", &http_response.text());

    
    Ok(())
}

fn keys_to_map(keys: Vec<JwkKey>) -> HashMap<String, JwkKey> {
    let mut keys_as_map = HashMap::new();
    for key in keys {
        keys_as_map.insert(String::clone(&key.kid), key);
    }
    keys_as_map
}

// helper functions
fn hash_password(password: &String) -> String {
    // let mut hasher = Sha3::sha3_256();
    // hasher.input_str(password);
    // hasher.result_str()
    password.clone()
}

// db functions
fn create_user(conn: &SqliteConnection, u: &UserCreateInfo) {
    use petbook::schema::users::dsl::*;
    use petbook::schema::users::dsl::id;
    use petbook::schema::userauth::dsl::*;

    let user: User = User {
        name: u.name.clone(),
        email: u.email.clone(),
        age: u.age};
    
    diesel::insert_into(users)
        .values(user)
        .execute(conn)
        .expect("Error creating user!");

    let user_entity: UserEntity = users
        .order(id.desc())
        .limit(1)
        .load::<UserEntity>(conn)
        .expect("Error fetchin new user!")
        .remove(0);
    
    let hashpw: String = hash_password(&u.password);

    let auth_info: UserAuth = UserAuth{
        user_id: user_entity.id,
        password_hash: hashpw};
    
    diesel::insert_into(userauth)
        .values(auth_info)
        .execute(conn)
        .expect("Error create auth_info!");

}

fn fetch_user_by_id(conn: &SqliteConnection, uid: i32) -> Option<UserEntity> {
    use petbook::schema::users::dsl::*;

    let mut matching_users: Vec<UserEntity> = users
        .filter(id.eq(uid))
        .load::<UserEntity>(conn)
        .expect("Error loading users!");
    if matching_users.len() == 0 {
        None
    }
    else {
        Some(matching_users.remove(0))
    }
}

fn fetch_user_by_email(conn: &SqliteConnection, user_email: &str) -> Option<UserEntity> {
    use petbook::schema::users::dsl::*;
    let mut matching_users: Vec<UserEntity> = users
        .filter(email.eq(user_email))
        .load::<UserEntity>(conn)
        .expect("Error loading users!");
    if matching_users.len() == 0 {
        None
    }
    else {
        Some(matching_users.remove(0))
    }
}

fn fetch_all_users(conn: &SqliteConnection) -> Vec<UserEntity> {
    use petbook::schema::users::dsl::*;
    users
        .order(id)
        .load::<UserEntity>(conn)
        .expect("Error loading users!")
}

fn fetch_user_auth_by_userid(conn: &SqliteConnection, uid: i32) -> Option<UserAuthEntity> {
    use petbook::schema::userauth::dsl::*;
    let mut matching_userauths: Vec<UserAuthEntity> = userauth
        .filter(user_id.eq(uid))
        .load::<UserAuthEntity>(conn)
        .expect("Error loading userauth!");
    if matching_userauths.len() == 0 {
        None
    }
    else {
        Some(matching_userauths.remove(0))
    }
}

// routes
#[get("/user/create")]
fn user_add() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("user_create", &context)
}

#[post("/user/create", data="<user_create_info>")]
fn user_add_post(conn: DbConn, user_create_info: Form<UserCreateInfo>) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    create_user(&conn, &user_create_info);
    Template::render("user_create_suc", &context)
}

// #[get("/users")]
// fn users(conn: DbConn) -> Template {
//     let users = fetch_all_users(&conn);
//     let mut context: HashMap<&str, Vec<UserEntity>> = HashMap::new();
//     context.insert("users", users);
//     Template::render("users", &context)
// }

#[get("/user")]
fn user_main(conn: DbConn, mut cookies: Cookies) -> Option<Template> {
    let maybe_usercookie = cookies.get_private("user_id");
    match maybe_usercookie {
        Some(usercookie) => {
            let userid = usercookie.value().parse::<i32>().unwrap();
            let maybe_user = fetch_user_by_id(&conn, userid);
            match maybe_user {
                Some(user) => {Some(Template::render("user_main", user))}
                None => None
            }
        }
        None => None
    }
}


#[get("/user/data")]
fn user_data(conn: DbConn, mut cookies: Cookies) -> Option<Template> {
    let maybe_usercookie = cookies.get_private("user_id");
    match maybe_usercookie {
        Some(usercookie) => {
            let userid = usercookie.value().parse::<i32>().unwrap();
            let maybe_user = fetch_user_by_id(&conn, userid);
            match maybe_user {
                Some(user) => {Some(Template::render("user_data", user))}
                None => None
            }
        }
        None => None
    }
}

#[get("/user/login")]
fn user_login() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("user_login", &context)
}

#[post("/user/login", data="<login_info>")]
fn user_login_post(conn: DbConn, login_info: Form<LoginInfo>, mut cookies: Cookies)
                   -> Option<Redirect> {
    let maybe_user = fetch_user_by_email(&conn, &login_info.email);
    match maybe_user {
        Some(user) => {
            let maybe_auth = fetch_user_auth_by_userid(&conn, user.id);
            match maybe_auth {
                Some(auth_info) => {
                    let hash = hash_password(&login_info.password);
                    if hash == auth_info.password_hash {
                        cookies.add_private(Cookie::new(
                            "user_id", user.id.to_string()));
                        Some(Redirect::to(uri!(user_main)))
                    } else {
                        None
                    }
                }
                None => None
            }
        }
        None => None
    }
}

#[post("/user/googlelogin", data="<glogin_info>")]
fn user_login_google(conn: DbConn, glogin_info: Form<GoogleLoginInfo>, mut cookies: Cookies)
                     -> Result<()> {
    let token: String = glogin_info.idtoken.clone();
    println!("token: {}", &token);

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
    
    let decoded_token = decode::<Claims>(&token,
                                         &DecodingKey::from_rsa_components(&key.n, &key.e),
                                         &Validation::new(Algorithm::RS256)).expect("validation error");

    // let decoded_token = match decoded_token {
    //     Ok(decoded_token) => decoded_token,
    //     Err(error) => panic!("Problem with decoding {:?}",error),
    // };
    
    println!("{:?}",decoded_token);
    Ok(())
}

#[post("/user/fblogin", data="<fblogin_info>")]
fn user_login_facebook(conn: DbConn, fblogin_info: Form<FacebookLoginInfo>, mut cookies: Cookies)
                     -> Result<()> {
    let token: String = fblogin_info.idtoken.clone();
    println!("token: {}", &token);

    let data = fb_validate_token(&token);
    println!("data: {:?}", &data);

    Ok(())
}



#[get("/user/logout")]
fn user_logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to(uri!(user_login))
}

// main
fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount("/", routes![user_main,
                            user_data,
                            user_add,
                            user_add_post,
                            user_login,
                            user_login_post,
                            user_login_google,
                            user_login_facebook,
                            user_logout
        ])
        .launch();
}

