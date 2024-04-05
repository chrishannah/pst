use std::env::args;

use reqwest::Result;
use crate::config::load::load_config;

mod micropub;
mod config;

pub use crate::micropub::api;

#[tokio::main]
async fn main() -> Result<()> {
    let post_type = args().nth(1).expect("no post type provided");
    let input = args().nth(2).expect("no input provided");

    let mut post_status = "draft";
    if post_type == "post" { post_status = "published"; }

    let mut token = String::new();
    let config = load_config();
    match config {
        Ok(value) => token = value.token,
        Err(err) => println!("Error: {}", err),
    }

    let post = api::create_post(&input, &token, &post_status);
    println!("{}", post.await);

    Ok(())
}
