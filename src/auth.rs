use rocket::request;
use rocket::outcome::IntoOutcome;
use crate::models::UserEntity;

impl<'a, 'r> request::FromRequest<'a, 'r> for UserEntity {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<UserEntity, Self::Error> {
        let db = request.guard::<crate::db_sqlite::DbConn>()?;
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id| crate::db_sqlite::fetch_user_by_id(&db, id))
            .or_forward(())
    }

}