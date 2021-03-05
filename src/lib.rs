//! # Move Links
//! CLI utility to move (or rename) your files to a new location and 
//! redirect all of its symbolic links, to the new path (or name).

pub mod cli_utils;
pub mod paths;

pub struct Config {
    pub verbose: bool,
}
impl Config {
    pub fn new(verbose: bool) -> Config {
        Config { verbose }
    }
}
