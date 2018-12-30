#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::env;

use reqwest::{Body, Client};

#[derive(Debug, Deserialize)]
struct Response {
    message: Option<String>,
    html_url: Option<String>,
}

fn main() {
    let github_token =
        env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable must be set!");

    let client = Client::new();

    let body = Body::from(
        r#"{
        "tag_name": "v1.0.5",
        "name": "v1.0.5",
        "body": " # Insert gitmoji-changelog there"
    }"#,
    );

    let response = client
        .post("https://api.github.com/repos/fabienjuif/test/releases")
        .header("Authorization", format!("token {}", github_token))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .unwrap()
        // .text().unwrap();
        .json::<Response>()
        .unwrap();

    println!("response_body = {:?}", response);

    if let Some(html_url) = response.html_url {
        println!("You can see the release here: {}", html_url);
    }
}
