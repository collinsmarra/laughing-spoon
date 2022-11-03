use std::any::type_name;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[macro_use] extern crate diesel;

pub mod models;
pub mod schemas;


use crate::models::{NewPost, Post};


pub fn print_type<T>(_:T){
    println!("{:?}", type_name::<T>());
}

pub fn establich_connection() -> PgConnection{
    dotenv().ok();

    let database_uri = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set");

    PgConnection::establish(&database_uri)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_uri))
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post{

    use crate::schemas::posts;
    
    let new_post = NewPost{title, body};
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
