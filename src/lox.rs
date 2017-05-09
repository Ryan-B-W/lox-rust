extern crate clap;
extern crate regex;
extern crate chrono;

extern crate libc;
extern crate pentry;

use clap::ArgMatches;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Shell {
    Fish,
    Bash
}

#[derive(Debug)]
pub struct LoxArgs {
    show_timestamp: bool,
    show_index: bool,
}

#[derive(Debug)]
struct Command {
    time: i64,
    cmd: String,
}

#[derive(Debug)]
struct ShellHistory {
    history: Vec<Command>,
    shell: Shell
}

pub fn process_args(matches: ArgMatches) -> LoxArgs {
    LoxArgs {
        show_timestamp: match matches.occurrences_of("t") {
            1 => true,
            _ => false,
        },
        show_index: match matches.occurrences_of("n") {
            1 => true,
            _ => false,
        },
    }
}

fn get_parent_shell() -> String {
    let pid: i32;
    unsafe {
        pid = libc::getppid() as i32;
    }

    if let Ok(ps) = pentry::find(pid) {
        let prog_option = ps.path().unwrap().split("/").collect::<Vec<&str>>();

        match prog_option.last() {
            Some(&v) => return v.to_owned(),
            _ => panic!("Unable to get shell name"),
        };
    } else {
        panic!("Unable to find shell PID")
    }
}

fn bash_history() -> ShellHistory {
    let home_directory = env!("HOME");
    let bash_history_path = home_directory.to_owned() + "/.bash_history";

    let mut file = match File::open(bash_history_path.to_string()) {
        Ok(v) => v,
        Err(_) => panic!("Fish file not found"),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => panic!("Unable to read file"),
    };

    return ShellHistory {
        shell: Shell::Bash,
        history: contents
            .as_str()
            .split("\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| {
                Command {
                    cmd : String::from(x),
                    time : -1
                }
            })
            .collect()
    }
}

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

fn fish_history() -> ShellHistory {
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

pub fn lox_main(matches: ArgMatches) {
    use self::chrono::prelude::*;

    let args: LoxArgs = process_args(matches);
    let shell_history: ShellHistory = match get_parent_shell().as_ref() {
        "fish" => fish_history(),
        "bash" => bash_history(),
        _ => panic!(format!("Unsupported shell: {}",  get_parent_shell()))
    };

    let mut idx = 0;

    for item in shell_history.history {
        let timestamp = match shell_history.shell {
          Shell::Fish => match args.show_timestamp {
              true => format!("{}\t", NaiveDateTime::from_timestamp(item.time, 0)),
              false => String::from(""),
          },
          _ => String::from("")
        };

        let index = match args.show_index {
            true => format!("{}\t", idx),
            false => String::from(""),
        };

        println!("{}{}{}", index, timestamp, item.cmd);
        idx += 1;
    }
}
