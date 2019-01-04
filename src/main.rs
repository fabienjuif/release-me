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
mod packagers;
mod repository;
mod service;

use crate::packagers::Packagers;
use crate::repository::Repository;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let matches = cli::parse_args();
    let release = matches.value_of("release").unwrap();

    let path = matches.value_of("path").unwrap();
    let mut repository = Repository::new(path, release);
    let changelog = repository.changelog(matches.is_present("print-authors"));
    let repository_name = repository.name();

    let packagers = Packagers::from(path);

    packagers.bump_all();

    return;
    if matches.is_present("dry-run") {
        println!(
            "---------- dry-run ---------
Changelog:
________ changelog ________
{}
_______ !changelog! _______
Repository name: {}
Packagers found: {}
--------- !dry-run! --------",
            changelog,
            repository_name,
            packagers
                .which()
                .iter()
                .fold(String::from(""), |acc, curr| format!(
                    "{}\n  - {}",
                    acc, curr
                ),),
        );
        return;
    }

    service::check();

    repository.commit_tag_push();

    service::publish(&repository_name, &release, &changelog);
}
