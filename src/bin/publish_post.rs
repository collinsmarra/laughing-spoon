use self::models::Post;
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;



fn main(){
    use self::schemas::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establich_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
        .expect(&format!("Unable to find post <{}>", id));

    println!("Published post <{}>", post.title);
}
