use reqwest::ClientBuilder;
use std::time::Duration;

const MICRO_PUB_API: &str = "https://micro.blog/micropub";

pub async fn create_post(input: &str, draft: bool, token: &str) -> String {
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new()
        .timeout(timeout)
        .build()
        .expect("cannot build client");

    let mut post_status = "published";
    if draft {
        post_status = "draft";
    }

    let params = [
        ("h", "entry"),
        ("content", &input),
        ("post-status", &post_status),
    ];

    let response = client
        .post(MICRO_PUB_API)
        .form(&params)
        .bearer_auth(token)
        .send()
        .await
        .expect("error during api call");

    println!("status: {}", response.status());

    let text = response
        .text()
        .await
        .expect("error parsing response");
    return text;
}
