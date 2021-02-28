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

    pub fn insert_dog(&self, dog: &Dog) -> QueryResult<DogEntity> {
        use crate::schema::dogs::dsl::*;
        use crate::schema::dogs::dsl::id;

        diesel::insert_into(dogs)
            .values(dog)
            .execute(&self.0)?;

        let dog_entity: DogEntity = dogs
            .order(id.desc())
            .first(&self.0)?;

        Ok(dog_entity)
   }

    pub fn update_dog(&self, dog_entity: &DogEntity) -> QueryResult<DogEntity> {
        use crate::schema::dogs::dsl::*;
        use crate::schema::dogs::dsl::id;

        diesel::update(dogs.filter(id.eq(dog_entity.id)))
            .set(dog_entity)
            .execute(&self.0)?;

        let updated_dog_entity: DogEntity =
            dogs.filter(id.eq(dog_entity.id))
            .first(&self.0)?;

        Ok(updated_dog_entity)
    }

    pub fn fetch_dogs(&self, user: &UserEntity) -> QueryResult<Vec<DogEntity>> {
        use crate::schema::dogs::dsl::*;
        dogs
            .filter(owner_id.eq(user.id))
            .load::<DogEntity>(&self.0)

    }

    pub fn fetch_dog_by_id(&self, dogid: i32) -> QueryResult<DogEntity> {
        use crate::schema::dogs::dsl::*;
        dogs
            .filter(id.eq(dogid))
            .first::<DogEntity>(&self.0)

    }

    pub fn fetch_dog_breeds(&self) -> QueryResult<Vec<DogBreedEntity>> {
        use crate::schema::dog_breeds::dsl::*;
        dog_breeds.load::<DogBreedEntity>(&self.0)
    }

}
