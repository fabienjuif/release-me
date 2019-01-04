use std::fs;
use std::path::Path;
use regex::Regex;

use serde_json::{self, Value};

lazy_static! {
    static ref RE_REMOVE_V: Regex = Regex::new(r"v?(.*)").unwrap();
    static ref RE_VERSION_NPM: Regex = Regex::new("(.*\"version\" ?: ?)(\".*?\")(.*)").unwrap();
    static ref RE_VERSION_CARGO: Regex = Regex::new("(version ?= ?)(\".*?\")").unwrap();
}

pub struct Packagers {
    packagers: Vec<&'static str>,
    files: Vec<String>
}

impl Packagers {
    pub fn from(path: &str) -> Packagers {
        let mut packagers = vec![];
        let mut files = vec![];

        if Path::new(&format!("{}/package.json", path)).exists() {
            packagers.push("npm");
            files.push(format!("{}/package.json", path));
        }

        if Path::new(&format!("{}/Cargo.toml", path)).exists() {
            packagers.push("cargo");
            files.push(format!("{}/Cargo.toml", path));
        }

        Packagers {
            packagers,
            files,
        }
    }

    pub fn which(&self) -> &Vec<&str> {
        &self.packagers
    }

    pub fn bump_all(&self, release: &str) {
        let release = RE_REMOVE_V.captures(release).unwrap().get(1).unwrap().as_str();

        for (packager, path) in self.packagers.iter().zip(self.files.iter()) {
            let mut content = fs::read_to_string(path).expect("Can't read file :(");

            match packager {
                &"npm" => {
                    let replacement = format!("$1\"{}\"$3", release);
                    let after = RE_VERSION_NPM.replace(&content, replacement.as_str()).into_owned();
                    fs::write(path, after).unwrap();
                },
                &"cargo" => {
                    let replacement = format!("$1\"{}\"", release);
                    let after = RE_VERSION_CARGO.replace(&content, replacement.as_str()).into_owned();
                    fs::write(path, after).unwrap();
                }
                _ => panic!("Unknown packager: {}", packager),
            }
        }
    }
}
