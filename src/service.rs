use std::env;

use reqwest::{Body, Client};

#[derive(Debug, Deserialize)]
struct Response {
    message: Option<String>,
    html_url: Option<String>,
}

// TODO: should take repository as parameter to retrieve which service is used (github or an other)
pub fn check() {
    env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable must be set!");
}

// TODO: returns something and let main.rs print messages
// TODO: use repository as a parameter to retrieve name and release and changelog alone ?
pub fn publish(repository_name: &str, release: &str, changelog: &str) {
    let github_token =
        env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable must be set!");

    println!("Releasing (github)...");

    let body = json!({
        "tag_name": release,
        "name": release,
        "body": changelog,
    });
    let body = body.to_string();
    let body = Body::from(body);

    let client = Client::new();
    let response = client
        .post(&format!(
            "https://api.github.com/repos/{}/releases",
            repository_name
        ))
        .header("Authorization", format!("token {}", github_token))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .unwrap()
        .json::<Response>()
        .unwrap();

    println!("response_body = {:?}", response);

    if let Some(html_url) = response.html_url {
        println!("You can see the release here: {}", html_url);
    }
}
