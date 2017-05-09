extern crate clap;
extern crate regex;
extern crate chrono;

extern crate libc;
extern crate pentry;

use clap::ArgMatches;

use lib::Shell;
use shells;

#[derive(Debug)]
pub struct LoxArgs {
    show_timestamp: bool,
    show_index: bool,
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

pub fn lox_main(matches: ArgMatches) {
    use self::chrono::prelude::*;

    let args: LoxArgs = process_args(matches);
    let shell_history = match get_parent_shell().as_ref() {
        "fish" => shells::fish::history(),
        "bash" => shells::bash::history(),
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
