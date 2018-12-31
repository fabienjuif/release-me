#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

use std::env;

use gitmoji_changelog::Changelog;
use reqwest::{Body, Client};

#[derive(Debug, Deserialize)]
struct Response {
    message: Option<String>,
    html_url: Option<String>,
}

fn main() {
    let github_token =
        env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable must be set!");

    let changelog = Changelog::from("../gitmoji-changelog", None)
        .keep_last_version_only()
        .to_markdown(Some("v1.0.6"), true);

    let body = json!({
        "tag_name": "v1.0.6",
        "name": "v1.0.6",
        "body": changelog,
    });
    let body = Body::from(body.to_string());

    let client = Client::new();
    let response = client
        .post("https://api.github.com/repos/fabienjuif/test/releases")
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
