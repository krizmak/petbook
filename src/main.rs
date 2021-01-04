#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use tera::Context;

use petbook::db_sqlite::DbConn;
use petbook::models::{UserEntity};
use petbook::auth_password::{LoginInfo, UserCreateInfo};
use petbook::auth_facebook::{FacebookLoginInfo, FacebookCreateInfo};
use petbook::auth_google::{GoogleLoginInfo, GoogleCreateInfo};
use petbook::auth::{AuthenticationError, UserCreationError};

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

#[post("/user/create", data = "<user_create_info>")]
fn user_add_post(db: DbConn, user_create_info: Form<UserCreateInfo>, cookies: Cookies)
    -> Result<Template, UserCreationError> {
    petbook::auth::create_user(db, &user_create_info.into_inner(), cookies)?;
    let context: HashMap<&str, &str> = HashMap::new();
    Ok(Template::render("user_create_suc", &context))
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

#[post("/user/login", data = "<login_info>")]
fn user_login_post(
    db: DbConn,
    login_info: Form<LoginInfo>,
    cookies: Cookies
) -> LoginResponse {
    let authentication_result = petbook::auth::authenticate_user(db, &login_info.into_inner(),cookies);
    match authentication_result {
        Ok(_) => LoginResponse::Redirect(Redirect::to(uri!(user_main))),
        Err(AuthenticationError::Failed) => LoginResponse::Redirect(Redirect::to(uri!(user_login))),
        Err(AuthenticationError::FailedWithEmail(email)) => LoginResponse::Err(format!("Wrong login info for: {}",email)),
        Err(AuthenticationError::InternalError(msg)) => LoginResponse::Err(format!("Error during login: {}", msg))
    }
}

#[post("/user/login_google", data = "<login_info>")]
fn user_login_google(
    db: DbConn,
    login_info: Form<GoogleLoginInfo>,
    cookies: Cookies
) -> LoginResponse {
    let login_info_inner = login_info.into_inner();
    let authentication_result = petbook::auth::authenticate_user(db, &login_info_inner, cookies);
    match authentication_result {
        Ok(_) => LoginResponse::Redirect(Redirect::to(uri!(user_main))),
        Err(AuthenticationError::FailedWithEmail(email)) => {
            let mut context = Context::new();
            context.insert("email", &email);
            context.insert("idtoken", &login_info_inner.idtoken);
            let ctx = context.into_json();
            return LoginResponse::Template(Template::render("user_create_google", &ctx));
        }
        Err(AuthenticationError::InternalError(msg)) => LoginResponse::Err(format!("Error during glogin: {}", msg)),
        Err(_) => LoginResponse::Err(format!("Unknown error during login"))
    }
}

#[post("/user/create_google", data = "<user_create_info>")]
fn user_create_google(db: DbConn, user_create_info: Form<GoogleCreateInfo>, cookies: Cookies)
    -> Result<Redirect, UserCreationError> {
    petbook::auth::create_user(db, &user_create_info.into_inner(), cookies)?;
    Ok(Redirect::to(uri!(user_main)))
}

#[post("/user/login_facebook", data = "<fblogin_info>")]
fn user_login_facebook(
    db: DbConn,
    fblogin_info: Form<FacebookLoginInfo>,
    cookies: Cookies,
) -> LoginResponse {
    let login_info_inner = fblogin_info.into_inner();
    let authentication_result = petbook::auth::authenticate_user(db, &login_info_inner, cookies);
    match authentication_result {
        Ok(_) => LoginResponse::Redirect(Redirect::to(uri!(user_main))),
        Err(AuthenticationError::FailedWithEmail(email)) => {
            let mut context = Context::new();
            context.insert("email", &email);
            context.insert("idtoken", &login_info_inner.idtoken);
            let ctx = context.into_json();
            return LoginResponse::Template(Template::render("user_create_facebook", &ctx));
        }
        Err(AuthenticationError::InternalError(msg)) => LoginResponse::Err(format!("Error during login: {}", msg)),
        Err(_) => LoginResponse::Err(format!("Unknown error during login"))
    }
}

#[post("/user/create_facebook", data = "<user_create_info>")]
fn user_create_facebook(
    db: DbConn,
    user_create_info: Form<FacebookCreateInfo>,
    cookies: Cookies,
) -> Result<Redirect, UserCreationError> {
    petbook::auth::create_user(db, &user_create_info.into_inner(), cookies)?;
    Ok(Redirect::to(uri!(user_main)))
}

#[get("/user/logout")]
fn user_logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to(uri!(user_login))
}

#[get("/user/pets")]
fn user_pets(user: UserEntity) -> Option<Template> {
    Some(Template::render("user_pets", user))
}

#[get("/pets/<id>")]
fn pet_data(id: u32, user: UserEntity) -> Option<Template> {
    Some(Template::render("pet_data", user))
}

// main
fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount("/static",StaticFiles::from("static"))
        .mount(
            "/",
            routes![
                user_main,
                user_data,
                user_add,
                user_add_post,
                user_login,
                user_login_post,
                user_login_google,
                user_create_google,
                user_login_facebook,
                user_create_facebook,
                user_logout,
                user_pets,
                pet_data
            ],
        )
        .launch();
}
