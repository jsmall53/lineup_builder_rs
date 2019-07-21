use std::error::Error;
use csv;
use serde::{ Deserialize, Serialize };
use serde::de;

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
    id: i64,
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
