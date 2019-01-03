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

use git2::{Commit, Cred, ObjectType, PushOptions, Remote, RemoteCallbacks, Repository};
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

fn find_last_commit(repository: &Repository) -> Result<Commit, git2::Error> {
    let obj = repository.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn commit_tag_push(repository: &Repository, remote: &mut Remote, release: &str) {
    let statuses = repository.statuses(None).unwrap();
    let mut index = repository.index().unwrap();
    for status in statuses.iter() {
        // TODO: check if merge! -> Error
        let path = Path::new(status.path().unwrap());
        index.add_path(path).unwrap();
    }
    index.write().unwrap();
    let oid = index.write_tree_to(&repository).unwrap();
    let parent_commit = find_last_commit(&repository).unwrap();
    let tree = repository.find_tree(oid).unwrap();
    let signature = repository.signature().unwrap();

    // commit
    println!("Committing...");
    let commit_oid = repository
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            &format!(":bookmark: {}", release),
            &tree,
            &[&parent_commit],
        )
        .unwrap();

    // tag
    println!("Tagging...");
    let commit_obj = repository.find_object(commit_oid, None).unwrap();
    repository
        .tag_lightweight(release, &commit_obj, true)
        .unwrap();

    // push
    println!("Pushing...");
    let mut callbacks = RemoteCallbacks::new();
    // look at https://github.com/rust-lang/cargo/blob/6a7672ef5344c1bb570610f2574250fbee932355/src/cargo/sources/git/utils.rs#L409-L617
    callbacks.credentials(|_url, user_name, _t| {
        // println!("{}-{:?}-{:?}", first, second, t);
        // if t.is_ssh_key() {
        // println!("Using ssh agent for: {}", second.unwrap());
        // Cred::ssh_key_from_agent(second.unwrap())
        Cred::ssh_key(
            user_name.unwrap(),
            None,
            Path::new(&format!(
                "{}/.ssh/id_rsa",
                env::var("HOME").expect("HOME environment variable must be set!")
            )),
            None,
        )
        // }

        // println!("Using default creds: {:?}", t);
        // Cred::default()
    });
    // remote
    //     .connect_auth(Direction::Push, Some(callbacks), None)
    //     .unwrap();
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);
    remote
        .push(
            &["refs/heads/master:refs/heads/master"],
            Some(&mut push_options),
        )
        .unwrap();
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
    let repository_name = String::from(repository_name);

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

    commit_tag_push(&repository, &mut remote, &release);

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
