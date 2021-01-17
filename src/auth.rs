use rocket::request;
use rocket::outcome::IntoOutcome;
use crate::models::UserEntity;
use crate::db_sqlite::DbConn;
use rocket::http::{Cookies, Cookie};

pub mod facebook;
pub mod google;
pub mod password;

pub enum AuthenticationError {
    FailedWithEmail(String),
    Failed,
    InternalError(String)
}

pub trait UserAuthenticator {
    fn authenticate(&self, db: &DbConn) -> Result<UserEntity, AuthenticationError>;
}

pub fn authenticate_user<T: UserAuthenticator>(
    db: DbConn,
    login_info: &T,
    mut cookies: Cookies
) -> Result<UserEntity, AuthenticationError> {
    let result = login_info.authenticate(&db);
    if let Result::Ok(ref user) = result {
        cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    }
    return result;
}

#[derive(Debug)]
pub enum UserCreationError {
    FailedAlreadyExists,
    InternalError(String),
}

pub trait UserCreator {
    fn create(&self, db: &DbConn) -> Result<UserEntity, UserCreationError>;
}

pub fn create_user<T: UserCreator>(
    db: DbConn,
    create_info: &T,
    mut cookies: Cookies
) -> Result<UserEntity, UserCreationError> {
    let result= create_info.create(&db);
    if let Result::Ok(ref user) = result {
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
            .and_then(
                |id| match db.get_user_by_id(id) {
                    Ok(u) => Some(u),
                    _ => None
            })
            .or_forward(())
    }

}

