use std::fs;
use std::path::Path;

pub struct Packagers {
    path: String,
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
            path: String::from(path),
            packagers,
            files,
        }
    }

    pub fn which(&self) -> &Vec<&str> {
        &self.packagers
    }

    pub fn bump_all(&self) {
        for (packager, path) in self.packagers.iter().zip(self.files.iter()) {
            let file = fs::read_to_string(path).expect("Can't read file :(");

            match packager {
                &"npm" => {
                    println!("TODO: parse {}", file);
                    println!("TODO: bump version {}", path);
                    println!("TODO: save {}", path);
                },
                &"cargo" => {
                    println!("TODO: parse {}", file);
                    println!("TODO: bump version {}", path);
                    println!("TODO: save {}", path);
                }
                _ => panic!("Unknown packager: {}", packager),
            }
        }
        // TODO:
        // parse JSON
        // Bump version
        // rewrite file
    }
}
