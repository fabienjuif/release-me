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

## Caution!!
This is the start point of this project, I didn't publish anything yet and this is **NOT READY TO USE OR TEST**!

## Roadmap
 - [ ] Use gitmoji-changelog to create the latest release changelog
 - [ ] Let the user change its changelog text
 - [ ] Publish to github
 - [ ] Publish to npm
 - [ ] Publish to cargo

## Commands
This project use a `Makefile`, here are the main targets:
  - `package`: build the docker image
  - `ci`: build the project (dev mode) and check clippy and rustfmt

You can still use cargo if you want to, eg building the release version with: `cargo build --release`
