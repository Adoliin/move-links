use move_links::Config;
mod cli_utils;
mod paths;
use paths::Paths;

use clap::{App, Arg};
use std::process;

fn main() {
    let matches = get_matches();
    let links_dir_raw = matches.value_of("LINKS_DIRECTORY").unwrap();
    let src_path_raw = matches.value_of("SOURCE").unwrap();
    let dest_path_raw = matches.value_of("DEST").unwrap();
    let verbose = match matches.occurrences_of("VERBOSE") {
        0 => false,
        _ => true,
    };
    let paths = Paths::new(links_dir_raw, src_path_raw, dest_path_raw).unwrap_or_else(|err| {
        println!("Error in arguments: {}", err);
        process::exit(1);
    });

    let config = Config::new(verbose);

    let links = cli_utils::find_links(&paths.src_path, &paths.links_dir, &config);
    match links {
        Some(links) => cli_utils::redirect_links(&links, &paths.dest_path, &config),
        None => (),
    };
    cli_utils::move_link(&paths.src_path, &paths.dest_path, &config);
}

fn get_matches() -> clap::ArgMatches<'static> {
    App::new("Move Links")
        .version("0.1.0")
        .visible_alias("do-stuff")
        .author("Ben Mefteh F. <benmeft0@gmail.com>")
        .about(
            "Move (or rename) SOURCE to DEST, and redirect all ot its symbolic \
                links inside of LINKS_DIRECTORY",
        )
        .arg(
            Arg::with_name("SOURCE")
                .help("Source file or directory")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("DEST")
                .help("Destination (output) file or directory")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("LINKS_DIRECTORY")
                .help("Directory in which to search for symbolic links of SOURCE")
                .required(true)
                .short("-d")
                .long("links-dir")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("VERBOSE")
                .help("Show what is move-link is executing under the hood")
                .short("-v")
                .long("verbose"),
        )
        .get_matches()
}
