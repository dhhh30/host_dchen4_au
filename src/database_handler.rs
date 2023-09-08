use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;



//Acquire connection pool for sqlite to avoid errors
//during moments of high read requestrs
pub fn acquire_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    //could change this to env config but stringy for now
    let url = "database.db";
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Could not build connection pool")
}

//Same as above, but this time it maintains an in memory connection pool
//for authentication tokening purposes
pub fn acquire_authentication_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Could not create authentication database")
}