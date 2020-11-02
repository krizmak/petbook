#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::collections::HashMap;

// use rocket::http::RawStr;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;

use diesel::prelude::*;
use hello_rust::schema::users;
use hello_rust::models::{User, UserEntity};

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

fn _establish_connection() -> SqliteConnection {

    let database_url = "data/data.sqlite";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_user(conn: &SqliteConnection, u: &User) {
    diesel::insert_into(users::table)
        .values(u)
        .execute(conn)
        .expect("Error creating user!");
}

fn fetch_user_by_id(conn: &SqliteConnection, uid: i32) -> UserEntity {
    use hello_rust::schema::users::dsl::*;
    let matching_users = users.filter(id.eq(uid))
        .order(name)
        .load::<UserEntity>(conn)
        .expect("Error loading users!");
    matching_users[0].clone()
}

fn fetch_user_by_name(conn: &SqliteConnection, user_name: &str) -> Vec<UserEntity> {
    use hello_rust::schema::users::dsl::*;
    users
        .filter(name.eq(user_name))
        .order(name)
        .load::<UserEntity>(conn)
        .expect("Error loading users!")
}

fn fetch_all_users(conn: &SqliteConnection) -> Vec<UserEntity> {
    use hello_rust::schema::users::dsl::*;
    users
        .order(id)
        .load::<UserEntity>(conn)
        .expect("Error loading users!")
}


#[get("/useradd")]
fn user_add() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("useradd", &context)
}

#[post("/useradd", data="<user>")]
fn user_add_post(conn: DbConn, user: Form<User>) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    create_user(&conn, &user);
    Template::render("useradded", &context)
}

#[get("/users")]
fn users(conn: DbConn) -> Template {
    let users = fetch_all_users(&conn);
    let mut context: HashMap<&str, Vec<UserEntity>> = HashMap::new();
    context.insert("users", users);
    println!("{:?}",Json(&context));
    Template::render("users", &context)
}


#[get("/users/<uid>")]
fn user_by_id(conn: DbConn, uid: i32) -> Template {
    let user = fetch_user_by_id(&conn, uid);
    println!("{:?}",Json(&user));
    Template::render("user",user)
}

#[get("/users/<name>", rank = 2)]
fn user_by_name(conn: DbConn, name: String) -> String {
    let users = fetch_user_by_name(&conn, &name);
    format!("Hello, {}, {:?}!", name, users)
}

// #[get("/hello/<name>/<age>/<cool>")]
// fn hello(name: String, age: u8, cool: bool) -> String {
//     if cool {
//         format!("You're a cool {} year old, {}!", age, name)
//     } else {
//         format!("{}, we need to talk about your coolness.", name)
//     }
// }

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount("/", routes![users,
                            user_by_id,
                            user_by_name,
                            user_add,
                            user_add_post])
        .launch();
}

