use std::fs::{ File };
use std::io::{ BufReader, Read };
use serde::{ Deserialize, Serialize };

use crate::category_mapper::{ CategoryMapper };
use crate::common::{ BuilderState, RosterSlot };

/// Just an intermediary struct for serializing/deserialize this data
#[derive(Debug, Deserialize, Serialize)]
struct Contest {
    salary_cap: u32,
    slots: Vec<RosterSlot>,
    // these are only here for serialization
    sport_type: String,
    contest_type: String,
    dfs_provider: String,
}

pub fn load_contest(file_path: &str, builder_state: &mut BuilderState) {
    let file = File::open(file_path)
                    .expect(&format!("failed to open contest template file: {}", file_path));
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let size: usize = match buf_reader.read_to_string(&mut contents) {
        Ok(size) => size,
        Err(_err) => 0
    };
    let contest: Contest = serde_json::from_str(&contents)
                            .expect("failed to parse contest template file");

    builder_state.salary_cap = Some(contest.salary_cap);
    builder_state.roster_slots = Some(contest.slots);
}

