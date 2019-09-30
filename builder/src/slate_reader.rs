use std::boxed::{ Box };
use std::collections::{ HashMap, HashSet };
use std::error::Error;
use std::fs::{ File };
use std::rc::{ Rc };
use csv;
use serde::{ Deserialize, Serialize };
use crate::common::{ BuilderState, Player };
use crate::category_mapper;
use crate::player_pool::PlayerPool;

// TODO: figure out this interface!
pub trait SlateReader {
    fn read(&self);
}

pub fn read_slate(file_path: &str, provider: &str, builder_state: &mut BuilderState, category_map: &HashMap<String, u32>) -> Result<(), &'static str> {
    let mut reader = csv::Reader::from_path(file_path).unwrap();
    if provider.to_lowercase() == "draft_kings" {
        return read_dk_row(&mut reader, builder_state, category_map);
    } else if provider.to_lowercase() == "fanduel" {
        return read_fanduel_row(&mut reader, builder_state, category_map);
    }
    Ok(())
}

fn read_dk_row(reader: &mut csv::Reader<File>, builder_state: &mut BuilderState, category_map: &HashMap<String, u32>) -> Result<(), &'static str> {
    let mut player_data_list: Vec<Player> = Vec::new();
    for result in reader.deserialize::<DKDataRow>() {
        match result {
            Ok(record) => {
                let category_keys: Vec<&str> = record.roster_position.split('/').collect();
                let mut categories: HashSet<u32> = HashSet::new();
                for key in category_keys {
                    let category = match category_map.get(key) {
                        Some(val) => Some(val),
                        None => {
                            println!("no value found for key '{:?}'", key);
                            None
                        }
                    };
                    categories.insert(*category.unwrap());
                }

                let mut player = Player {
                    id: record.id,
                    name: record.name,
                    team: record.teamabbrev,
                    position: record.position,
                    price: record.salary,
                    projected_points: record.avg_points_per_game,
                    categories: categories,
                };
                // TODO: fix this, tie the salary multiplier to the roster slot?
                if &record.roster_position == "CPT" {
                    player.projected_points *= 1.5;
                    player.price = ((player.price as f64) * 1.5) as u32; // this is bad lmao
                }
                player_data_list.push(player);
            },
            Err(_err) => return Err("error parsing csv")
        }
    }
    let player_pool = PlayerPool::new(player_data_list, true);
    builder_state.player_pool = Some(player_pool);
    Ok(())
}

fn read_fanduel_row(reader: &mut csv::Reader<File>, builder_state: &mut BuilderState, category_map: &HashMap<String, u32>) -> Result<(), &'static str> {
    let mut player_data_list: Vec<Player> = Vec::new();
    let mut id: u64 = 1;
    for result in reader.deserialize::<FanDuelDataRow>() {
        match result {
            Ok(record) => {
                let mut category_keys: Vec<&str> = record.position.split('/').collect();
                append_flex_for_fanduel(&mut category_keys); // this is kind of a hack, but Fanduel's data format sucks
                let mut categories: HashSet<u32> = HashSet::new();
                for key in category_keys {
                    let category = match category_map.get(key) {
                        Some(val) => Some(val),
                        None => {
                            println!("No value found for key {}", key);
                            None
                        }
                    };
                    categories.insert(*category.unwrap());
                }
                
                let mut player = Player {
                    id: id,
                    name: record.nickname,
                    team: record.team,
                    position: record.position,
                    price: record.salary,
                    projected_points: record.fantasy_points_per_game,
                    categories: categories,
                };

                // TODO: fix this, tie the salary multiplier to the roster slot?
                // if &record.position == "CPT" {
                //     player.projected_points *= 1.5;
                // }
                id += 1;
                player_data_list.push(player);
            },
            Err(_err) => {
                println!("{:?}", _err);
                return Err("error parsing csv");
            },
        }
    }
    let player_pool = PlayerPool::new(player_data_list, true);
    builder_state.player_pool = Some(player_pool);
    Ok(())
}

fn append_flex_for_fanduel(category_keys: &mut Vec<&str>) {
    for key in &*category_keys {
        if *key == "RB" || *key == "WR" || *key == "TE" {
            category_keys.push("FLEX");
            break;
        }
    }
}

/// This data row is DK specific, need to implement a trait or something so I can use multiple different data mappings
#[derive(Debug, Deserialize, Serialize)]
struct DKDataRow {
    #[serde(rename = "Position")]
    position: String,
    
    #[serde(rename = "Name + ID")]
    name_id: String,
    
    #[serde(rename = "Name")]
    name: String,
    
    #[serde(rename = "ID")]
    id: u64,
    
    #[serde(rename = "Roster Position")]
    roster_position: String,
    
    #[serde(rename = "Salary")]
    salary: u32,
    
    #[serde(rename = "Game Info")]
    game_info: String,
    
    #[serde(rename = "TeamAbbrev")]
    teamabbrev: String,
    
    #[serde(rename = "AvgPointsPerGame")]
    avg_points_per_game: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FanDuelDataRow {
    #[serde(rename = "Position")]
    position: String,
    
    #[serde(rename = "Nickname")]
    nickname: String,

    #[serde(rename = "FPPG")]
    fantasy_points_per_game: f64,

    #[serde(rename = "Salary")]
    salary: u32,

    #[serde(rename = "Game")]
    game: String,

    #[serde(rename = "Team")]
    team: String,

    #[serde(rename = "Opponent")]
    opponent: String,
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::category_mapper::choose_category_mapper;
    
//     #[test]
//     fn read_slate() {
//         // not really a unit test
//         let mut reader = read_slate("../data/DKSalaries.csv");
//         reader.read().unwrap();
//         let player_pool = reader.get_player_pool(choose_category_mapper("nba").unwrap());
//         assert!(player_pool.len() > 0);
//     }
// }
