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
use petbook::user::models::{UserEntity, User, Address};
use petbook::dog::models::{Dog, Log};
use petbook::dog::forms::{DogForm, LogForm};
use petbook::user::auth::password::{LoginInfo, UserCreateInfo};
use petbook::user::auth::facebook::{FacebookLoginInfo, FacebookCreateInfo};
use petbook::user::auth::google::{GoogleLoginInfo, GoogleCreateInfo};
use petbook::user::auth::{AuthenticationError, UserCreationError};
use petbook::user::forms::UserForm;

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
    Template::render("user/user_create", &context)
}

#[post("/user/create", data = "<user_create_info>")]
fn user_add_post(db: DbConn, user_create_info: Form<UserCreateInfo>, cookies: Cookies)
    -> Result<Template, UserCreationError> {
    petbook::user::auth::create_user(db, &user_create_info.into_inner(), cookies)?;
    let context: HashMap<&str, &str> = HashMap::new();
    Ok(Template::render("user/user_create_suc", &context))
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
    Some(Template::render("user/user_main", user))
}

#[get("/user/data")]
fn user_data(db: DbConn, user_entity: UserEntity) -> Option<Template> {
    let mut context = Context::new();
    let (user_id, user) = User::from_entity(user_entity);
    let (_, address) = user.get_address(&db);
    context.insert("user", &UserForm::from_objects(&user, &address));
    context.insert("user_id", &user_id);
    Some(Template::render("user/user_data", context.into_json()))
}

#[post("/user/data", data = "<user_form>")]
fn user_update_post(db: DbConn, user_entity: UserEntity, user_form: Form<UserForm>) -> Option<Template> {
    let (user_id, mut user) = User::from_entity(user_entity);
    let (maybe_address_id, mut address) = user.get_address(&db);
    user_form.to_objects(&mut user, &mut address);
    println!("Update user: {:?}", &user);
    match maybe_address_id {
        None => {
            let (address_id, _) = address.insert(&db).unwrap();
            user.address_id = Some(address_id);
        },
        Some(address_id) => {Address::update(address_id, &address, &db);}
    };
    User::update( user_id, &user, &db);

    let mut context = Context::new();
    context.insert("user_id", &user_id);
    context.insert("user", &UserForm::from_objects(&user, &address));
    Some(Template::render("user/user_data", context.into_json()))
}

#[get("/user/login")]
fn user_login() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("user/user_login", &context)
}

#[post("/user/login", data = "<login_info>")]
fn user_login_post(
    db: DbConn,
    login_info: Form<LoginInfo>,
    cookies: Cookies
) -> LoginResponse {
    let authentication_result = petbook::user::auth::authenticate_user(db, &login_info.into_inner(), cookies);
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
    let authentication_result = petbook::user::auth::authenticate_user(db, &login_info_inner, cookies);
    match authentication_result {
        Ok(_) => LoginResponse::Redirect(Redirect::to(uri!(user_main))),
        Err(AuthenticationError::FailedWithEmail(email)) => {
            let mut context = Context::new();
            context.insert("email", &email);
            context.insert("idtoken", &login_info_inner.idtoken);
            let ctx = context.into_json();
            return LoginResponse::Template(Template::render("user/user_create_google", &ctx));
        }
        Err(AuthenticationError::InternalError(msg)) => LoginResponse::Err(format!("Error during glogin: {}", msg)),
        Err(_) => LoginResponse::Err(format!("Unknown error during login"))
    }
}

#[post("/user/create_google", data = "<user_create_info>")]
fn user_create_google(db: DbConn, user_create_info: Form<GoogleCreateInfo>, cookies: Cookies)
    -> Result<Redirect, UserCreationError> {
    petbook::user::auth::create_user(db, &user_create_info.into_inner(), cookies)?;
    Ok(Redirect::to(uri!(user_main)))
}

#[post("/user/login_facebook", data = "<fblogin_info>")]
fn user_login_facebook(
    db: DbConn,
    fblogin_info: Form<FacebookLoginInfo>,
    cookies: Cookies,
) -> LoginResponse {
    let login_info_inner = fblogin_info.into_inner();
    let authentication_result = petbook::user::auth::authenticate_user(db, &login_info_inner, cookies);
    match authentication_result {
        Ok(_) => LoginResponse::Redirect(Redirect::to(uri!(user_main))),
        Err(AuthenticationError::FailedWithEmail(email)) => {
            let mut context = Context::new();
            context.insert("email", &email);
            context.insert("idtoken", &login_info_inner.idtoken);
            let ctx = context.into_json();
            return LoginResponse::Template(Template::render("user/user_create_facebook", &ctx));
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
    petbook::user::auth::create_user(db, &user_create_info.into_inner(), cookies)?;
    Ok(Redirect::to(uri!(user_main)))
}

#[get("/user/logout")]
fn user_logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to(uri!(user_login))
}

#[get("/user/pets")]
fn user_pets(db: DbConn, user: UserEntity) -> Result<Template, UserCreationError> {
    let dogs = db.fetch_dogs(&user).map_err(|_| UserCreationError::InternalError("Google error".to_owned()))?;
    let mut context = Context::new();
    context.insert("dogs", &dogs);
    Ok(Template::render("user_pets", &context.into_json()))
}

#[get("/pets/<id>")]
fn pet_data(db: DbConn, id: i32, user: UserEntity) -> Result<Template,UserCreationError> {
    let (dog_id, dog) = Dog::get(id,&db).map_err(|_| UserCreationError::InternalError("Google error".to_owned()))?;
    if dog.owner_id == user.id {
        let mut context = Context::new();
        //let dog_breeds = db.fetch_dog_breeds().map_err(|_| UserCreationError::InternalError("Google error".to_owned()))?;
        context.insert("dog", &DogForm::from_dog(Some(&dog), &db));
        context.insert("dog_id", &dog_id);
        //context.insert("breeds", &dog_breeds);
        Ok(Template::render("pet/data", context.into_json()))
    }
    else {
        Err(UserCreationError::InternalError("other error".to_owned()))
    }
}

#[post("/user/pet/<id>/update", data = "<dog_form>")]
fn pet_update_post(db: DbConn, id: i32, user: UserEntity, dog_form: Form<DogForm>) -> Result<Template,UserCreationError> {
    let (dog_id, dog) = Dog::get(id, &db).map_err(|_| UserCreationError::InternalError("Google error".to_owned()))?;
    if dog.owner_id == user.id {
        let updated_dog = dog_form.to_dog(&user);
        Dog::update( dog_id, &updated_dog, &db);
        let mut context = Context::new();
        context.insert("dog_id", &dog_id);
        context.insert("dog", &DogForm::from_dog(Some(&updated_dog), &db));
        Ok(Template::render("pet/data", context.into_json()))
    }
    else {
        Err(UserCreationError::InternalError("other error".to_owned()))
    }
}

#[get("/user/pet/add")]
fn pet_add_get(db: DbConn, _user: UserEntity) -> Result<Template,UserCreationError> {
    let mut context = Context::new();
    context.insert("dog", &DogForm::from_dog(None, &db));
    Ok(Template::render("pet/add", context.into_json()))
}

#[post("/user/pet/add", data = "<dog_form>")]
fn pet_add_post(db: DbConn, user: UserEntity, dog_form : Form<DogForm>) -> Redirect {
    let dog = dog_form.to_dog(&user);
    println!("adding: {:?}", &dog);
    dog.insert(&db);
    Redirect::to(uri!(user_pets))
}

#[get("/pet/<id>/logs")]
fn pet_logs_get(db: DbConn, id: i32, user: UserEntity) -> Result<Template,UserCreationError> {
    let (dog_id, dog) = Dog::get(id, &db).map_err(|_| UserCreationError::InternalError("Google error".to_owned()))?;
    if dog.owner_id == user.id {
        let mut context = Context::new();
        let logs = Log::get_pet_logs(&dog_id, &db);
        context.insert("dog_id",&dog_id);
        context.insert("logs", &logs);
        Ok(Template::render("pet/logs", &context.into_json()))
    } else {
        Err(UserCreationError::InternalError("other error".to_owned()))
    }
}

#[get("/pet/<id>/logs/add")]
fn pet_logs_add_get(db: DbConn, id: i32, user: UserEntity) -> Result<Template,UserCreationError> {
    let (dog_id, dog) = Dog::get(id, &db).map_err(|_| UserCreationError::InternalError("Google error".to_owned()))?;
    if dog.owner_id == user.id {
        let mut context = Context::new();
        context.insert("dog_id", &dog_id);
        context.insert("log", &LogForm::from_object(None));
        Ok(Template::render("pet/log/add", context.into_json()))
    } else {
        Err(UserCreationError::InternalError("other error".to_owned()))
    }
}

#[post("/pet/<id>/logs/add", data = "<log_form>")]
fn pet_logs_add_post(db: DbConn, id: i32, user: UserEntity, log_form : Form<LogForm>) -> Result<Redirect,UserCreationError> {
    let (dog_id, dog) = Dog::get(id, &db).map_err(|_| UserCreationError::InternalError("Google error".to_owned()))?;
    if dog.owner_id == user.id {
        let mut log = Log::new(Some(dog_id));
        log_form.to_object(& mut log);
        println!("adding: {:?}", &log);
        log.insert(&db);
        Ok(Redirect::to(uri!(pet_logs_get: dog_id)))
    }
    else {
        Ok(Redirect::to(uri!(user_pets)))
    }
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
                user_update_post,
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
                pet_data,
                pet_update_post,
                pet_add_get,
                pet_add_post,
                pet_logs_get,
                pet_logs_add_get,
                pet_logs_add_post
            ],
        )
        .launch();
}
