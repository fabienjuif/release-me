#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate clap;

use std::env;

use gitmoji_changelog::Changelog;
use reqwest::{Body, Client};

mod cli;

#[derive(Debug, Deserialize)]
struct Response {
    message: Option<String>,
    html_url: Option<String>,
}

fn main() {
    let github_token =
        env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable must be set!");

    let matches = cli::parse_args();
    let release = matches.value_of("release").unwrap();

    let changelog = Changelog::from(matches.value_of("path").unwrap(), None)
        .keep_last_version_only()
        .to_markdown(Some(release), matches.is_present("print-authors"));

    if matches.is_present("dry-run") {
        println!("---------- dry-run ---------\n{}\n--------- !dry-run! --------", changelog);
        return;
    }

    let body = json!({
        "tag_name": release,
        "name": release,
        "body": changelog,
    });
    let body = body.to_string();
    let body = Body::from(body);

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
