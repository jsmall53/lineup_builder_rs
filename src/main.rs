extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

mod lineup_optimizer;

use std::error::Error;
use std::process;

use clap::{ App, Arg };

fn run(file_path: &str) -> Result<(), Box<Error>> {
    let mut reader = lineup_optimizer::SlateDataReader::new(&file_path);
    reader.read()?;
    let player_pool = reader.get_player_pool();
    
    for player in &player_pool {
        println!("{:?}\n", player);
    }
    Ok(())
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

    let config = matches.value_of("config").unwrap_or("default.conf");
    let input_file = matches.value_of("INPUT_FILE").unwrap(); // this is a required parameter
    if let Err(err) = run(&input_file) {
        println!("{}", err);
        process::exit(1);
    }
}
