use crate::models::{User, UserEntity};
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;
use crate::diesel::ExpressionMethods;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

// db functions
pub fn create_user(conn: &DbConn, user: &User) -> UserEntity {
    use crate::schema::users::dsl::*;
    use crate::schema::users::dsl::id;

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

pub fn fetch_user_by_google_id(conn: &DbConn, gid: &str)
                                -> Option<UserEntity> {
    use crate::schema::users::dsl::*;

    let mut matching_users: Vec<UserEntity> = users
        .filter(google_id.eq(gid))
        .load::<UserEntity>(&conn.0)
        .expect("Error loading user!");
    if matching_users.len() == 0 {
        None
    }
    else {
        Some(matching_users.remove(0))
    }
}

pub fn fetch_user_by_facebook_id(conn: &DbConn, fbid: &str)
                                  -> Option<UserEntity> {
    use crate::schema::users::dsl::*;

    let mut matching_users: Vec<UserEntity> = users
        .filter(facebook_id.eq(fbid))
        .load::<UserEntity>(&conn.0)
        .expect("Error loading user!");
    if matching_users.len() == 0 {
        None
    }
    else {
        Some(matching_users.remove(0))
    }
}

