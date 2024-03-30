use std::env::args;

use reqwest::Result;
use crate::config::load::load_config;

mod micropub;
mod config;

pub use crate::micropub::api;

#[tokio::main]
async fn main() -> Result<()> {
    let input = args().nth(1).expect("no input provided");

    let mut token = String::new();
    let config = load_config();
    match config {
        Ok(value) => token = value.token,
        Err(err) => println!("Error: {}", err),
    }

    let post = api::create_post(&input, true, &token);
    println!("{}", post.await);

    Ok(())
}
