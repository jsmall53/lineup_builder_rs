#![allow(dead_code)]
#![allow(unused_imports)]

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

use std::error::Error;
use std::process;
use clap::{ App, Arg };
use builder::builder::{ Builder, Lineup };

fn run(file_path: &str) -> Result<Lineup, &'static str> {
    let builder = Builder::new("./resources/game_templates/");
    let result = builder.provider("draft_kings")
                        .sport("nba")
                        .contest("showdown")
                        .slate(file_path)
                        .build().expect("optimizer build step failed")
                        .optimize();
    // match result {
    //     Ok(ref lineup) => println!("{}", lineup.to_string()),
    //     Err(msg) => println!("Error creating lineup: {}", msg),
    // };

    result
}

fn main() {
    let matches = App::new("lineup optimizer")
                        .version("0.0.1")
                        .author("Jordan Small")
                        .about("Optimizes DFS lineups")
                        .arg(Arg::with_name("INPUT_FILE")
                                .help("Sets the input file to use")
                                .required(true)
                                .index(1))
                        .arg(Arg::with_name("provider")
                                .short("p")
                                .help("set the data provider"))
                        .get_matches();

    let _config = matches.value_of("config").unwrap_or("default.conf");
    let input_file = matches.value_of("INPUT_FILE").unwrap(); // this is a required parameter
    match run(&input_file) {
        Ok(lineup) => println!("{}", lineup.to_string()),
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
