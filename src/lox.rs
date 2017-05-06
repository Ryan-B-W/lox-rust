extern crate clap;
extern crate yaml_rust;
extern crate regex;

use clap::ArgMatches;
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
    time: i64,
    cmd: String
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

fn clean (line: &str) -> String {
    if line.matches(":").count() > 1 {
        let newline = String::from(line);
        use self::regex::Regex;
        let re = Regex::new(r"^\- \w+: (.*)$").unwrap();

        if re.is_match( newline.as_str() ) {
            let cap = re.captures(newline.as_str()).unwrap();
            let out = format!( "- cmd: \"{}\"", &cap[1]);
            return out;
        } else {
            panic!("Bad match!");
        }
    } else {
        return line.to_string();
    }
}

fn fish_history () -> FishHistory {
    use self::yaml_rust::{YamlLoader, YamlEmitter, Yaml};

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

    let mut sanitized : String = contents
            .as_str()
            .split("\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| {
                clean(x)
            })
            .collect::<Vec<String>>()
            .join("\n");

    let parsed_history = match YamlLoader::load_from_str(sanitized.as_str()) {
        Ok(v) => v,
        Err(e) => panic!("Unable to parse fish history")
    };

    let mut out : Vec<FishCommand> = Vec::new();
    &match parsed_history[0].as_vec() {
        Some(col) => {
            for item in col {
                out.push(FishCommand {
                    time: item["when"].as_i64().unwrap(),
                    cmd: String::from(item["cmd"].as_str().unwrap())
                });
            }
        },
        None => panic!("Unable to parse fish history")
    };

    return FishHistory {
        history : out
    }
}

pub fn lox_main(matches: ArgMatches) {
    let args : FishArgs = process_args(matches);
    let fish_history : FishHistory = fish_history();

    for item in fish_history.history {
        let timestamp = match args.show_timestamp {
            true => "WORDS",
            false => ""
        };

        println!("{}{}", timestamp, item.cmd);
    }
}