#[allow(unused)]
use diesel::prelude::*;
use diesel::Queryable;
use diesel::Insertable;

use crate::schemas::posts;


#[derive(Queryable)]
pub struct Post{
    pub id: i32,
    pub title: String,
    pub body: String,
    pub publishable: bool,
}


#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a>{
    pub title: &'a str,
    pub body: &'a str,
}
