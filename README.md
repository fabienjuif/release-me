# release-me
> Create your release with ease!

<br />
<p style="text-align: center" align="center">
  <a href="https://circleci.com/gh/fabienjuif/release-me/tree/master">
    <img src="https://img.shields.io/circleci/project/github/fabienjuif/release-me/master.svg" />
  </a>
  <a href="https://crates.io/crates/release-me">
    <img src="https://img.shields.io/crates/v/release-me.svg" />
  </a>
  <a href="https://hub.docker.com/r/fabienjuif/release-me">
    <img src="https://img.shields.io/badge/docker--image-fabienjuif%2Frelease--me-blue.svg" />
    <img src="https://img.shields.io/microbadger/image-size/fabienjuif%2Frelease-me.svg" />
  </a>
  <br />
  [<a href="https://docs.rs/crate/release-me">documentation</a>]
  [<a href="https://github.com/fabienjuif/release-me">repository</a>]
</p>
<br />

## Purpose
The main purpose is to publish my lib with more ease.

I want to have one simple command that:
 - Generate a changelog from the last tag to HEAD
 - Create the github release
 - Publish to npm (or crates.io)

## Roadmap
 - [x] Use gitmoji-changelog to create the latest release changelog
 - [ ] Let the user change its changelog text
 - [x] Publish to github
 - [ ] Publish to npm
 - [ ] Publish to cargo

## Try it
### With Docker 🐳!
```sh
## set your github token
## - the github token needs to have "repo" privileges
## - you can create a new token here: https://github.com/settings/tokens/new
export GITHUB_TOKEN="your token"

## try it
docker run --rm -e GITHUB_TOKEN=${GITHUB_TOKEN} -v ${PWD}:/repo fabienjuif/release-me /repo --release <your_version>
# ex: docker run --rm -e GITHUB_TOKEN=${GITHUB_TOKEN} -v ${PWD}:/repo fabienjuif/release-me /repo --release

## to see which options you can use:
docker run --rm -e GITHUB_TOKEN=${GITHUB_TOKEN} fabienjuif/release-me --help
```

### With cargo
```sh
## install it
cargo install release-me

# maybe you should reset your env here (relaunch your terminal or type `zsh` (or `bash`))

## set your github token
## - the github token needs to have "repo" privileges
## - you can create a new token here: https://github.com/settings/tokens/new
export GITHUB_TOKEN="your token"

## try it
release-me . --release #<your_version>
# ex: release-me . --release v0.1.0

## to see which options you can use:
release-me --help
```


## Commands
This project use a `Makefile`, here are the main targets:
  - `package`: build the docker image
  - `ci`: build the project (dev mode) and check clippy and rustfmt

You can still use cargo if you want to, eg building the release version with: `cargo build --release`
