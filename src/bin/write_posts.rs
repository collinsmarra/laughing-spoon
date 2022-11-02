use diesel_demo::*;
use std::io::{stdin, Read};

fn main(){
    let connection = &mut establich_connection();
    
    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!(
        "\nOk! Lets's write {} (Press {} when finished) \n",
        title, EOF
            );
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);
    println!("\nsaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

// #[cfg(windows)]
// const EOF: str = "CTRL+Z";
