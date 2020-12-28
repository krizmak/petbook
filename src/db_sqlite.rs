use crate::models::{User, UserEntity};
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;
use crate::diesel::ExpressionMethods;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

impl DbConn {
    pub fn insert_user(&self, user: &User) -> UserEntity {
        use crate::schema::users::dsl::*;
        use crate::schema::users::dsl::id;

        diesel::insert_into(users)
            .values(user)
            .execute(&self.0)
            .expect("Error creating user!");

        let user_entity: UserEntity = users
            .order(id.desc())
            .limit(1)
            .load::<UserEntity>(&self.0)
            .expect("Error fetching new user!")
            .remove(0);

        return user_entity;
    }

    pub fn get_user_by_id(&self, uid: i32) -> Option<UserEntity> {
        use crate::schema::users::dsl::*;

        let mut matching_users: Vec<UserEntity> = users
            .filter(id.eq(uid))
            .load::<UserEntity>(&self.0)
            .expect("Error loading users!");
        if matching_users.len() == 0 {
            None
        } else {
            Some(matching_users.remove(0))
        }
    }

    pub fn get_user_by_email(&self, user_email: &str) -> Option<UserEntity> {
        use crate::schema::users::dsl::*;
        let mut matching_users: Vec<UserEntity> = users
            .filter(email.eq(user_email))
            .load::<UserEntity>(&self.0)
            .expect("Error loading users!");
        if matching_users.len() == 0 {
            None
        } else {
            Some(matching_users.remove(0))
        }
    }

    pub fn get_all_users(&self) -> Vec<UserEntity> {
        use crate::schema::users::dsl::*;
        users
            .order(id)
            .load::<UserEntity>(&self.0)
            .expect("Error loading users!")
    }

    pub fn get_user_by_google_id(&self, gid: &str)
                                 -> Option<UserEntity> {
        use crate::schema::users::dsl::*;

        let mut matching_users: Vec<UserEntity> = users
            .filter(google_id.eq(gid))
            .load::<UserEntity>(&self.0)
            .expect("Error loading user!");
        if matching_users.len() == 0 {
            None
        } else {
            Some(matching_users.remove(0))
        }
    }

    pub fn get_user_by_facebook_id(&self, fbid: &str)
                                   -> Option<UserEntity> {
        use crate::schema::users::dsl::*;

        let mut matching_users: Vec<UserEntity> = users
            .filter(facebook_id.eq(fbid))
            .load::<UserEntity>(&self.0)
            .expect("Error loading user!");
        if matching_users.len() == 0 {
            None
        } else {
            Some(matching_users.remove(0))
        }
    }
}
