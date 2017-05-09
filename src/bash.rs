use std::fs::File;
use std::io::prelude::*;

use lib::{Command, Shell, ShellHistory};

pub fn history() -> ShellHistory {
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
