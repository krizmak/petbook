use rocket::request;
use rocket::outcome::IntoOutcome;
use crate::models::UserEntity;
use crate::db_sqlite::DbConn;
use rocket::http::{Cookies, Cookie};

pub enum AuthenticationResult {
    AuthenticatedUser(UserEntity),
    FailedWithEmail(String),
    Error(String),
}

pub trait UserAuthenticator {
    fn authenticate(&self, db: &DbConn) -> AuthenticationResult;
}

pub fn authenticate_user<T: UserAuthenticator>(
    db: DbConn,
    login_info: &T,
    mut cookies: Cookies
) -> AuthenticationResult {
    let result = login_info.authenticate(&db);
    if let AuthenticationResult::AuthenticatedUser(ref user) = result {
        cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    }
    return result;
}

pub enum UserCreationResult {
    User(UserEntity),
    Error(String),
}


pub trait UserCreator {
    fn create(&self, db: &DbConn) -> UserCreationResult;
}

pub fn create_user<T: UserCreator>(
    db: DbConn,
    create_info: &T,
    mut cookies: Cookies
) -> UserCreationResult {
    let result = create_info.create(&db);
    if let UserCreationResult::User(ref user) = result {
        cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    }
    return result;
}


impl<'a, 'r> request::FromRequest<'a, 'r> for UserEntity {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<UserEntity, Self::Error> {
        let db = request.guard::<crate::db_sqlite::DbConn>()?;
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id| db.get_user_by_id(id))
            .or_forward(())
    }

}

