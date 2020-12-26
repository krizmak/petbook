#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;

use tera::{Context};

use petbook::models::{User, UserEntity};
use petbook::db_sqlite::*;
use petbook::types::*;

#[derive(Debug, Responder)]
pub enum LoginResponse {
    Template(Template),
    Redirect(Redirect),
    Err(String),
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
fn user_main(user: UserEntity) -> Option<Template> {
    Some(Template::render("user_main", user))
}

#[get("/user/data")]
fn user_data(user: UserEntity) -> Option<Template> {
    Some(Template::render("user_data", user))
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
                    if Some(hash) == auth_info.password_hash {
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

#[post("/user/login_google", data="<glogin_info>")]
fn user_login_google(conn: DbConn, glogin_info: Form<GoogleLoginInfo>, mut cookies: Cookies)
                     -> LoginResponse {
    let token: String = glogin_info.idtoken.clone();
    println!("token: {}", &token);

    let claims = petbook::auth_google::decode_token(&token);

    let maybe_auth = fetch_user_auth_by_google_id(&conn, &claims.sub);
    if maybe_auth.is_none() {
        let new_user = User {
            email : claims.email,
            name : claims.name,
            age : None
        };
        let mut context = Context::new();
        context.insert("user", &new_user);
        context.insert("idtoken", &token);
        let ctx = context.into_json();
        println!("{:?}", &ctx);
        return LoginResponse::Template(Template::render("user_create_google", &ctx));
    } else {
        let user_auth = maybe_auth.unwrap();
        cookies.add_private(Cookie::new(
            "user_id", user_auth.user_id.to_string()));
        return LoginResponse::Redirect(Redirect::to(uri!(user_main)));
    };
}

#[post("/user/create_google", data="<gcreate_info>")]
fn user_create_google(conn: DbConn, gcreate_info: Form<GoogleCreateInfo>, mut cookies: Cookies)
                      -> Redirect {

    let new_user = google_create_user(&conn, &gcreate_info);
    cookies.add_private(Cookie::new(
                            "user_id", new_user.id.to_string()));
    Redirect::to(uri!(user_main))
}

#[post("/user/login_facebook", data="<fblogin_info>")]
fn user_login_facebook(conn: DbConn, fblogin_info: Form<FacebookLoginInfo>, mut cookies: Cookies)
                       -> LoginResponse {
    let token: String = fblogin_info.idtoken.clone();
    println!("token: {}", &token);

    let user_data= petbook::auth_facebook::decode_token(&token);

    let maybe_auth = fetch_user_auth_by_facebook_id(&conn, &user_data.id);
    if maybe_auth.is_none() {
        let new_user = User {
            email : user_data.email,
            name : user_data.name,
            age : None
        };
        let mut context = Context::new();
        context.insert("user", &new_user);
        context.insert("idtoken", &token);
        let ctx = context.into_json();
        println!("{:?}", &ctx);
        return LoginResponse::Template(Template::render("user_create_facebook", &ctx));
    } else {
        let user_auth = maybe_auth.unwrap();
        cookies.add_private(Cookie::new(
            "user_id", user_auth.user_id.to_string()));
        return LoginResponse::Redirect(Redirect::to(uri!(user_main)));
    };
}

#[post("/user/create_facebook", data="<fbcreate_info>")]
fn user_create_facebook(conn: DbConn, fbcreate_info: Form<FacebookCreateInfo>, mut cookies: Cookies)
                        -> Redirect {

    let new_user = facebook_create_user(&conn, &fbcreate_info);
    cookies.add_private(Cookie::new(
        "user_id", new_user.id.to_string()));
    Redirect::to(uri!(user_main))
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
                            user_create_google,
                            user_login_facebook,
                            user_create_facebook,
                            user_logout
        ])
        .launch();
}

