use std::collections::{ HashSet };
use std::error::Error;
use csv;
use serde::{ Deserialize, Serialize };
use crate::common::{ BuilderState, Player };
use crate::category_mapper::{ CategoryMapper };

// TODO: figure out this interface!
pub trait SlateReader {
    fn read(&self);
}

pub fn read_slate(file_path: &str, builder_state: &mut BuilderState, mapper: impl CategoryMapper) -> Result<(), Box<Error>> {
    let mut reader = csv::Reader::from_path(file_path).unwrap();
    let mut player_data_list: Vec<Player> = Vec::new();
    for result in reader.deserialize() {
        let record: SlateDataRow = result?;
        let categories: HashSet<u32> = mapper.map(&record.roster_position);
        let mut player = Player {
            id: record.id,
            name: record.name,
            price: record.salary,
            projected_points: record.avg_points_per_game,
            categories: categories,
        };
        if &record.roster_position == "CPT" {
            player.projected_points *= 1.5;
        }
        player_data_list.push(player);
    }
    builder_state.player_data_list = Some(player_data_list);
    Ok(())
}

/// This data row is DK specific, need to implement a trait or something so I can use multiple different data mappings
#[derive(Debug, Deserialize, Serialize)]
pub struct SlateDataRow {
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
