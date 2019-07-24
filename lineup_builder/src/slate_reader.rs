use std::collections::{ HashSet };
use std::error::Error;
use csv;
use serde::{ Deserialize, Serialize };
use crate::common::Player;
use crate::category_mapper::{ CategoryMapper };

// TODO: figure out this interface!
pub trait SlateReader {
    fn read(&self);
    fn file_path(&self);
}

pub struct SlateDataReader {
    file_path: String,
    player_data_list: Vec<SlateDataRow>,
}

impl SlateDataReader {
    pub fn new(file_path: &str) -> SlateDataReader {
        SlateDataReader {
            file_path: String::from(file_path),
            player_data_list: Vec::new(),
        }
    }

    pub fn read(&mut self) -> Result<(), Box<Error>> {
        let mut reader = csv::Reader::from_path(&self.file_path).unwrap();
        for result in reader.deserialize() {
            let record: SlateDataRow = result?;
            self.player_data_list.push(record);
        }
        Ok(())
    }

    pub fn get_player_pool(&self, mapper: impl CategoryMapper) -> Vec<Player> {
        let mut player_pool = Vec::new();
        for row in &self.player_data_list {
            let categories: HashSet<u32> = mapper.map(&row.roster_position);
            let player = Player::new(row.id, &row.name, row.salary, row.avg_points_per_game, categories);
            player_pool.push(player);
        }
        player_pool.sort_by(|a, b| a.partial_cmp(b).unwrap());
        player_pool
    }
}

/// This data row is DK specific, need to implement a trait or something so I can use multiple different data mappings
#[derive(Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::category_mapper::choose_category_mapper;
    
    #[test]
    fn read_slate() {
        // not really a unit test
        let mut reader = SlateDataReader::new("../data/DKSalaries.csv");
        reader.read();
        let player_pool = reader.get_player_pool(choose_category_mapper("nba").unwrap());
        assert!(player_pool.len() > 0);
    }
}
