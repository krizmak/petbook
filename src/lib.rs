#[macro_use]
extern crate diesel;

mod util;

pub mod schema;
pub mod models;

pub mod db_sqlite;
pub mod user;
pub mod dog;
pub mod widget;