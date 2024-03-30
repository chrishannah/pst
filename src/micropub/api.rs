use reqwest::ClientBuilder;
use serde::Deserialize;
use serde_json::from_str;
use std::time::Duration;

const MICRO_PUB_API: &str = "https://micro.blog/micropub";

#[derive(Deserialize)]
pub struct PostResponse {
    url: String,
    preview: String,
    edit: String
}

#[derive(Deserialize)]
pub struct PostError {
    error: String,
    error_description: String
}

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

    let status = response.status();
    let text = &response
        .text()
        .await
        .expect("error parsing response");

    if !status.is_success() {
        let post_error: PostError = from_str(&text).expect("Failed to parse error response");
        let error = format!("Failed to publish post ({}).\n{}", post_error.error, post_error.error_description);
        return error;
    }

    let post: PostResponse = from_str(&text).expect("Failed to parse response");
    let post_info = format!("Post published successfully.\nURL: {}\nPreview: {}\nEdit: {}", 
                            post.url, post.preview, post.edit);

    return post_info.to_string();
}
