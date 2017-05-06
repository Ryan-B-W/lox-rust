extern crate clap;
extern crate yaml_rust;

use clap::ArgMatches;
use self::yaml_rust::{YamlLoader, YamlEmitter};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct FishArgs {
    show_timestamp: bool,
    show_index: bool
}

#[derive(Debug)]
struct FishCommand {

}

#[derive(Debug)]
struct FishHistory {
    history: Vec<FishCommand>
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

fn fish_history () -> FishHistory {
    let home_directory = env!("HOME");
    let fish_history_path = home_directory.to_owned() + "/.local/share/fish/fish_history";
    let mut file = match File::open(fish_history_path.to_string()) {
        Ok(v) => v,
        Err(e) => panic!("Fish file not found")
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(v) => (),
        Err(e) => panic!("Unable to read file")
    };

    let parsed_history = match YamlLoader::load_from_str(contents.as_str()) {
        Ok(v) => v,
        Err(e) => panic!("Unable to parse fish history")
    };

    return FishHistory {
        history : vec![]
    }
}

pub fn lox_main(matches: ArgMatches) {
    let args : FishArgs = process_args(matches);
    let fish_history : FishHistory = fish_history();
}