use std::env;
use std::path::Path;

use git2::{
    Commit, Cred, ObjectType, PushOptions, Remote, RemoteCallbacks, Repository as GitRepository,
};
use gitmoji_changelog::Changelog;
use gitmoji_changelog::Error;
use regex::Regex;

lazy_static! {
    static ref RE_REMOTE_SSH: Regex = Regex::new(r"^[@.\w]*:([\w/-]+)\.?(git|.*)?").unwrap();
}

pub struct Repository {
    release: String,
    path: String,
    repository: GitRepository,
}

impl Repository {
    pub fn new(path: &str, release: &str) -> Repository {
        let repository = GitRepository::open(Path::new(path)).expect("Cannot open repository");

        Repository {
            repository,
            path: String::from(path),
            release: String::from(release),
        }
    }

    pub fn remote(&self) -> Remote {
        self.repository
            .find_remote("origin")
            .expect("Remote origin should exists!")
    }

    pub fn name(&self) -> String {
        let remote = self.remote();
        let remote_url = remote.url().expect("Remote origin should exists!");

        let name = RE_REMOTE_SSH
            .captures(remote_url)
            .expect("Could not find repository name in your \"remote origin\"");

        name.get(1)
            .expect("Could not find repository name in your \"remote origin\"")
            .as_str()
            .to_string()
    }

    pub fn commit_tag_push(&mut self) {
        let statuses = self.repository.statuses(None).unwrap();
        let mut index = self.repository.index().unwrap();
        for status in statuses.iter() {
            // TODO: check if merge! -> Error
            let path = Path::new(status.path().unwrap());
            index.add_path(path).unwrap();
        }
        index.write().unwrap();
        let oid = index.write_tree_to(&self.repository).unwrap();
        let parent_commit = self.find_last_commit().unwrap();
        let tree = self.repository.find_tree(oid).unwrap();
        let signature = self.repository.signature().unwrap();

        // commit
        println!("Committing...");
        let commit_oid = self
            .repository
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                &format!(":bookmark: {}", self.release),
                &tree,
                &[&parent_commit],
            )
            .unwrap();

        // tag
        println!("Tagging...");
        let commit_obj = self.repository.find_object(commit_oid, None).unwrap();
        self.repository
            .tag_lightweight(&self.release, &commit_obj, true)
            .unwrap();

        // push
        println!("Pushing...");
        let mut callbacks = RemoteCallbacks::new();

        // TODO: add more credentials
        // look at https://github.com/rust-lang/cargo/blob/6a7672ef5344c1bb570610f2574250fbee932355/src/cargo/sources/git/utils.rs#L409-L617
        callbacks.credentials(|_url, user_name, _t| {
            Cred::ssh_key(
                user_name.unwrap(),
                None,
                Path::new(&format!(
                    "{}/.ssh/id_rsa",
                    env::var("HOME").expect("HOME environment variable must be set!")
                )),
                None,
            )
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);
        self.remote()
            .push(
                &["refs/heads/master:refs/heads/master"],
                Some(&mut push_options),
            )
            .unwrap();
    }

    pub fn changelog(&self, print_authors: bool) -> Result<String, Error> {
        Changelog::from(&self.path, None)
            .keep_last_version_only()
            .to_markdown(Some(&self.release), print_authors)
    }

    fn find_last_commit(&self) -> Result<Commit, git2::Error> {
        let obj = self
            .repository
            .head()?
            .resolve()?
            .peel(ObjectType::Commit)?;
        obj.into_commit()
            .map_err(|_| git2::Error::from_str("Couldn't find commit"))
    }
}
