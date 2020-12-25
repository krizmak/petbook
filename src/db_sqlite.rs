use crate::types::{UserCreateInfo, GoogleCreateInfo, FacebookCreateInfo};
use crate::models::{User, UserEntity, UserAuth, UserAuthEntity};
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;
use crate::diesel::ExpressionMethods;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

// db functions
pub fn create_user(conn: &DbConn, u: &UserCreateInfo) {
    use crate::schema::users::dsl::*;
    use crate::schema::users::dsl::id;
    use crate::schema::userauth::dsl::*;

    let user: User = User {
        name: u.name.clone(),
        email: u.email.clone(),
        age: u.age};

    diesel::insert_into(users)
        .values(user)
        .execute(&conn.0)
        .expect("Error creating user!");

    let user_entity: UserEntity = users
        .order(id.desc())
        .limit(1)
        .load::<UserEntity>(&conn.0)
        .expect("Error fetching new user!")
        .remove(0);

    let hashpw: String = hash_password(&u.password);

    let auth_info: UserAuth = UserAuth{
        user_id: user_entity.id,
        password_hash: Some(hashpw),
        facebook_id: None,
        google_id: None,
    };

    diesel::insert_into(userauth)
        .values(auth_info)
        .execute(&conn.0)
        .expect("Error create auth_info!");

}

// db functions
pub fn google_create_user(conn: &DbConn, u: &GoogleCreateInfo) -> UserEntity {
    use crate::schema::users::dsl::*;
    use crate::schema::users::dsl::id;
    use crate::schema::userauth::dsl::*;

    let user: User = User {
        name: u.name.clone(),
        email: u.email.clone(),
        age: u.age};

    diesel::insert_into(users)
        .values(user)
        .execute(&conn.0)
        .expect("Error creating user!");

    let user_entity: UserEntity = users
        .order(id.desc())
        .limit(1)
        .load::<UserEntity>(&conn.0)
        .expect("Error fetchin new user!")
        .remove(0);

    let token: String = u.idtoken.clone();
    println!("token: {}", &token);
    let claims = crate::auth_google::decode_token(&token);

    let gid = claims.sub.clone();

    let auth_info: UserAuth = UserAuth{
        user_id: user_entity.id,
        password_hash: None,
        facebook_id: None,
        google_id: Some(gid),
    };

    diesel::insert_into(userauth)
        .values(auth_info)
        .execute(&conn.0)
        .expect("Error create auth_info!");

    return user_entity;
}

pub fn facebook_create_user(conn: &DbConn, u: &FacebookCreateInfo) -> UserEntity {
    use crate::schema::users::dsl::*;
    use crate::schema::users::dsl::id;
    use crate::schema::userauth::dsl::*;

    let user: User = User {
        name: u.name.clone(),
        email: u.email.clone(),
        age: u.age};

    diesel::insert_into(users)
        .values(user)
        .execute(&conn.0)
        .expect("Error creating user!");

    let user_entity: UserEntity = users
        .order(id.desc())
        .limit(1)
        .load::<UserEntity>(&conn.0)
        .expect("Error fetching new user!")
        .remove(0);

    let token: String = u.idtoken.clone();
    println!("token: {}", &token);
    let user_data = crate::auth_facebook::decode_token(&token);

    let fbid = user_data.id.clone();

    let auth_info: UserAuth = UserAuth{
        user_id: user_entity.id,
        password_hash: None,
        facebook_id: Some(fbid),
        google_id: None,
    };

    diesel::insert_into(userauth)
        .values(auth_info)
        .execute(&conn.0)
        .expect("Error create auth_info!");

    return user_entity;
}


pub fn fetch_user_by_id(conn: &DbConn, uid: i32) -> Option<UserEntity> {
    use crate::schema::users::dsl::*;

    let mut matching_users: Vec<UserEntity> = users
        .filter(id.eq(uid))
        .load::<UserEntity>(&conn.0)
        .expect("Error loading users!");
    if matching_users.len() == 0 {
        None
    }
    else {
        Some(matching_users.remove(0))
    }
}

pub fn fetch_user_by_email(conn: &DbConn, user_email: &str) -> Option<UserEntity> {
    use crate::schema::users::dsl::*;
    let mut matching_users: Vec<UserEntity> = users
        .filter(email.eq(user_email))
        .load::<UserEntity>(&conn.0)
        .expect("Error loading users!");
    if matching_users.len() == 0 {
        None
    }
    else {
        Some(matching_users.remove(0))
    }
}

pub fn fetch_all_users(conn: &DbConn) -> Vec<UserEntity> {
    use crate::schema::users::dsl::*;
    users
        .order(id)
        .load::<UserEntity>(&conn.0)
        .expect("Error loading users!")
}

pub fn fetch_user_auth_by_userid(conn: &DbConn, uid: i32) -> Option<UserAuthEntity> {
    use crate::schema::userauth::dsl::*;
    let mut matching_userauths: Vec<UserAuthEntity> = userauth
        .filter(user_id.eq(uid))
        .load::<UserAuthEntity>(&conn.0)
        .expect("Error loading userauth!");
    if matching_userauths.len() == 0 {
        None
    }
    else {
        Some(matching_userauths.remove(0))
    }
}

pub fn fetch_user_auth_by_google_id(conn: &DbConn, gid: &str)
                                -> Option<UserAuthEntity> {
    use crate::schema::userauth::dsl::*;
    let mut matching_userauths: Vec<UserAuthEntity> = userauth
        .filter(google_id.eq(gid))
        .load::<UserAuthEntity>(&conn.0)
        .expect("Error loading userauth!");
    if matching_userauths.len() == 0 {
        None
    }
    else {
        Some(matching_userauths.remove(0))
    }
}

pub fn fetch_user_auth_by_facebook_id(conn: &DbConn, fbid: &str)
                                  -> Option<UserAuthEntity> {
    use crate::schema::userauth::dsl::*;
    let mut matching_userauths: Vec<UserAuthEntity> = userauth
        .filter(facebook_id.eq(fbid))
        .load::<UserAuthEntity>(&conn.0)
        .expect("Error loading userauth!");
    if matching_userauths.len() == 0 {
        None
    }
    else {
        Some(matching_userauths.remove(0))
    }
}

// helper functions
pub fn hash_password(password: &String) -> String {
    // let mut hasher = Sha3::sha3_256();
    // hasher.input_str(password);
    // hasher.result_str()
    password.clone()
}

