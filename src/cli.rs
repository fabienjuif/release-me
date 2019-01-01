use clap::{App, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches<'static> {
    App::new("release-me")
    .version(crate_version!())
    .author("Fabien JUIF <fabien.juif@gmail.com>")
    .arg(
        Arg::with_name("print-authors")
            .short("a")
            .long("print-authors")
            .help("Print author for each commit")
            .takes_value(false),
    )
    .arg(
        Arg::with_name("dry-run")
            .long("dry-run")
            .help("Generate the changelog but doesn't create the release.")
            .takes_value(false),
    )
    .arg(
        Arg::with_name("path")
            .value_name("GIT_REPOSITORY_PATH")
            .help("Path to the git repository to parse")
            .required(true),
    )
    .arg(
        Arg::with_name("release")
            .short("r")
            .long("release")
            .help("Set a version to the release (latest tag to HEAD). If not set, the commits after the latest tag will not be printed to the changelog.")
            .takes_value(true)
            .required(true)
    )
    .get_matches()
}
