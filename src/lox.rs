extern crate clap;

use clap::ArgMatches;

#[derive(Debug)]
pub struct FishArgs {
    show_timestamp: bool,
    show_index: bool
}

pub fn process_args(matches: ArgMatches) -> FishArgs {
    FishArgs {
        show_timestamp : match matches.occurrences_of("t") {
            1 => true,
            _ => false
        },
        show_index : match matches.occurrences_of("n") {
            1 => true,
            _ => false
        }
    }
}