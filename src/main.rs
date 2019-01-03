#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate openssl_probe;

use std::env;
use std::path::Path;

use git2::{Commit, Index, IndexAddOption, ObjectType, Repository};
use gitmoji_changelog::Changelog;
use regex::Regex;
use reqwest::{Body, Client};

mod cli;

lazy_static! {
    static ref RE_REMOTE_SSH: Regex = Regex::new(r"^[@.\w]*:([\w/-]+)\.?(git|.*)?").unwrap();
}

#[derive(Debug, Deserialize)]
struct Response {
    message: Option<String>,
    html_url: Option<String>,
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let github_token =
        env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable must be set!");

    let matches = cli::parse_args();
    let release = matches.value_of("release").unwrap();

    let repository = matches.value_of("path").unwrap();
    let changelog = Changelog::from(repository, None)
        .keep_last_version_only()
        .to_markdown(Some(release), matches.is_present("print-authors"));

    let repository = Path::new(&repository);
    let repository = Repository::open(repository).unwrap();
    let mut remote = repository
        .find_remote("origin")
        .expect("Remote origin should exists!");
    let remote_url = remote.url().expect("Remote origin should exists!");
    let repository_name = RE_REMOTE_SSH
        .captures(remote_url)
        .expect("Could not find repository name in your \"remote origin\"");
    let repository_name = repository_name
        .get(1)
        .expect("Could not find repository name in your \"remote origin\"")
        .as_str();

    if matches.is_present("dry-run") {
        println!(
            "---------- dry-run ---------
Changelog:
________ changelog ________
{}
_______ !changelog! _______
Repository name: {}
--------- !dry-run! --------",
            changelog, repository_name,
        );
        return;
    }

    let statuses = repository.statuses(None).unwrap();
    let mut index = repository.index().unwrap();
    for status in statuses.iter() {
        // TODO: check if merge! -> Error
        let path = Path::new(status.path().unwrap());
        index.add_path(path);
    }
    index.write();
    let oid = index.write_tree_to(&repository).unwrap();
    let parent_commit = find_last_commit(&repository).unwrap();
    let tree = repository.find_tree(oid).unwrap();
    let signature = repository.signature().unwrap();
    repository
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            &format!(":bookmark: {}", release),
            &tree,
            &[&parent_commit],
        )
        .unwrap();
    remote.push(&["refs/heads/master:refs/heads/master"], None).unwrap();

    return;

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
