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
use clap::{ App, Arg, ArgMatches };
use builder::builder::{ Builder, Lineup };

fn run(matches: ArgMatches) -> Result<Vec<Lineup>, &'static str> {
    let input_file = matches.value_of("INPUT_FILE").unwrap(); // this is a required parameter
    let provider = matches.value_of("provider").unwrap();
    let contest_type = matches.value_of("contest-type").unwrap();
    let sport = matches.value_of("sport").unwrap();

    let builder = Builder::new("./resources/game_templates/");
    let result = builder.provider(provider)
                        .sport(sport)
                        .contest(contest_type)
                        .slate(input_file)
                        .build().expect("optimizer build step failed")
                        .optimize();
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
                            .default_value("draft_kings")
                            .help("set the data provider"))
                        .arg(Arg::with_name("contest-type")
                            .short("c")
                            .default_value("classic")
                            .help("the type of contest the lineups are for"))
                        .arg(Arg::with_name("sport")
                            .short("s")
                            .default_value("nfl")
                            .help("sets the sport type"))
                        .get_matches();

    let _config = matches.value_of("config").unwrap_or("default.conf");

    match run(matches) {
        Ok(lineups) => {
            for lineup in lineups {
                println!("{}", lineup.to_string())
            }
        },
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
