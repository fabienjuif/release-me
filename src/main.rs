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

mod cli;
mod repository;
mod service;

use crate::repository::Repository;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let matches = cli::parse_args();
    let release = matches.value_of("release").unwrap();

    let mut repository = Repository::new(matches.value_of("path").unwrap(), release);
    let changelog = repository.changelog(matches.is_present("print-authors"));
    let repository_name = repository.name();

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

    service::check();

    repository.commit_tag_push();

    service::publish(&repository_name, &release, &changelog);
}
