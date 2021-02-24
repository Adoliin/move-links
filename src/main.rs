//#![allow(unused)]
mod cli_utils;

mod paths;
use paths::Paths;

use clap::{App, Arg};
use std::process;

fn main() {
    let matches = get_matches();
    let links_dir_raw = matches.value_of("links_dir").unwrap();
    let src_path_raw = matches.value_of("src_path").unwrap();
    let dest_path_raw = matches.value_of("dest_path").unwrap();

    let paths = Paths::new(
        links_dir_raw, src_path_raw, dest_path_raw
    ).unwrap_or_else(|err| {
        println!("Error in arguments: {}", err);
        process::exit(1);
    });
    let links = cli_utils::find_links(&paths.src_path, &paths.links_dir);
    match links {
        Some(links) => cli_utils::redirect_links(&links, &paths.dest_path),
        None => {
            println!("No links were found in {}", links_dir_raw);
        }
    };
    cli_utils::move_link(&paths.src_path, &paths.dest_path);
}

fn get_matches() -> clap::ArgMatches<'static> {
    App::new("Move Links")
        .version("0.1")
        .author("Ben Mefteh F. <benmeft0@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("src_path")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("dest_path")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("links_dir")
                .help("Sets a custom config file")
                .required(true)
                .short("-d")
                .long("links-dir")
                .takes_value(true),
        )
        .get_matches()
}
