extern crate clap;
use clap::{Arg, App};

mod fish;
mod bash;
mod lox;
mod lib;

fn main() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

    let matches = App::new("Lox")
        .version(VERSION.unwrap_or("development"))
        .author("David Nuon <david@davidnuon.com>")
        .arg(Arg::with_name("t")
                 .short("t")
                 .help("Display timestamp next to command"))
        .arg(Arg::with_name("n")
                 .short("n")
                 .help("Display index next to command"))
        .get_matches();

    lox::lox_main(matches);
}
