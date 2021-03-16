use crate::models::{User, UserEntity};
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;
use crate::diesel::ExpressionMethods;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;
use diesel::QueryResult;
use crate::dog::models::{DogEntity, Dog, DogBreedEntity};

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

impl DbConn {
    pub fn insert_user(&self, user: &User) -> QueryResult<UserEntity> {
        use crate::schema::users::dsl::*;
        use crate::schema::users::dsl::id;

        diesel::insert_into(users)
            .values(user)
            .execute(&self.0)?;

        let user_entity: UserEntity = users
            .order(id.desc())
            .first(&self.0)?;

        Ok(user_entity)
    }

    pub fn get_user_by_id(&self, uid: i32) -> QueryResult<UserEntity> {
        use crate::schema::users::dsl::*;

        users
            .filter(id.eq(uid))
            .first::<UserEntity>(&self.0)
    }

    pub fn get_user_by_email(&self, user_email: &str) -> QueryResult<UserEntity> {
        use crate::schema::users::dsl::*;
        users
            .filter(email.eq(user_email))
            .first::<UserEntity>(&self.0)
    }

    pub fn get_user_by_google_id(&self, gid: &str) -> QueryResult<UserEntity> {
        use crate::schema::users::dsl::*;

        users
            .filter(google_id.eq(gid))
            .first::<UserEntity>(&self.0)
    }

    pub fn get_user_by_facebook_id(&self, fbid: &str) -> QueryResult<UserEntity> {
        use crate::schema::users::dsl::*;

        users
            .filter(facebook_id.eq(fbid))
            .first::<UserEntity>(&self.0)
    }

    pub fn get_all_users(&self) -> QueryResult<Vec<UserEntity>> {
        use crate::schema::users::dsl::*;
        users
            .order(id)
            .load::<UserEntity>(&self.0)
    }

    pub fn fetch_dogs(&self, user: &UserEntity) -> QueryResult<Vec<DogEntity>> {
        use crate::schema::dogs::dsl::*;
        dogs
            .filter(owner_id.eq(user.id))
            .load::<DogEntity>(&self.0)

    }

    pub fn fetch_dog_breeds(&self) -> QueryResult<Vec<DogBreedEntity>> {
        use crate::schema::dog_breeds::dsl::*;
        dog_breeds.load::<DogBreedEntity>(&self.0)
    }

}
