#[derive(Debug)]
pub enum UserAuthorizationError {
    NoUserFound,
    GoogleError,
    FacebookError,
}
