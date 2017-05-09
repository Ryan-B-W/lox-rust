use std::fs::File;
use std::io::prelude::*;

use lib::{Command, Shell, ShellHistory};

fn parse_fish_history(lines : Vec<&str>) -> Vec<Command> {
    #[derive(Debug)]
    enum State {
        INIT,
        COMMAND,
        TIME,
        PATH
    };

    let mut state : State = State::INIT;
    let mut idx : usize = 0;
    let num_lines : usize = lines.len();

    let mut last_command : String = String::from("");;
    let mut current_history : Vec<Command> = Vec::new();

    loop {
        if idx == num_lines - 1{
            break;
        }

        if lines[idx + 1].len() == 0 {
            break;
        }

        println!("{:?}\t{}", state, lines[idx]);

        match state {
            State::INIT => {
                state = State::COMMAND
            },
            State::COMMAND => {
                last_command = lines[idx]
                    .replace("- cmd:", "")
                    .trim()
                    .to_string();

                idx += 1;
                let when_position = lines[idx].find("when:");
                match when_position {
                    Some(_) => state = State::TIME,
                    None => panic!(format!("Unexpected line: {}", lines[idx]))
                }
            },
            State::TIME => {
                let time = lines[idx]
                    .replace("when:", "")
                    .trim()
                    .parse::<i64>()
                    .unwrap();

                idx += 1;
                let path_position = lines[idx].find("paths:").is_some();
                let cmd_position = lines[idx].find("- cmd:").is_some();
                match (path_position, cmd_position) {
                    (true, false) => state = State::PATH,
                    (false, true) => {
                        current_history.push(Command {
                            cmd: last_command.to_string(),
                            time: time
                        });
                        state = State::COMMAND;
                    },
                    (_, _) => panic!("@TIME: Bad parse state: {:?} {:?}", lines[idx], state)
                }
            },
            State::PATH => {
                idx += 1;
                let path_position = lines[idx].find("paths:").is_some();
                let cmd_position = lines[idx].find("- cmd:").is_some();
                match (path_position, cmd_position) {
                    (false, false) => state = State::PATH,
                    (false,  true) => state = State::COMMAND,
                    (true,  false) => state = State::PATH,
                    (true,   true) => panic!("@PATH: Bad parse state: {} {:?}", lines[idx], state)
                }
            }
        };
    }

    return current_history;
}

pub fn history() -> ShellHistory {
    let home_directory = env!("HOME");
    let fish_history_path = home_directory.to_owned() + "/.local/share/fish/fish_history";
    let mut file = match File::open(fish_history_path.to_string()) {
        Ok(v) => v,
        Err(_) => panic!("Fish file not found"),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => panic!("Unable to read file"),
    };

    let lines = contents
        .as_str()
        .split("\n")
        .collect::<Vec<&str>>();

    return ShellHistory {
        history: parse_fish_history(lines),
        shell: Shell::Fish
    };
}