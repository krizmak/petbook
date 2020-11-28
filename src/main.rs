#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::collections::HashMap;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;

use diesel::prelude::*;
use serde::{Deserialize};

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
                            user_logout
        ])
        .launch();
}

