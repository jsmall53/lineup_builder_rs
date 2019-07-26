use std::cmp::{ Ordering };
use std::collections::{ HashSet };
use serde::{ Deserialize, Serialize };

use crate::category_mapper::CategoryMapper;

pub enum DFSProvider {
    DraftKings,
    FanDuel,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SportType {
    nfl,
    nba,
    mlb,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LineupSlot {
    // id: i32,
    pub name: String,
    pub key: String,
    pub count: u32,
    pub salary_multiplier: f64,
    pub point_multiplier: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LineupContext {
    pub sport_type: SportType,
    pub slots: Vec<LineupSlot>,
    pub salary_cap: u32,
}

impl LineupContext {
    pub fn calculate_category_count(&mut self, mapper: impl CategoryMapper) -> Vec<u32> {
        let mut category_counts: Vec<u32> = vec!(0; 100);
        for slot in &self.slots {
            let index_map = mapper.map(&slot.key);
            let category_index = index_map.iter().take(1).next().unwrap(); // a slot will only ever have one position key
            category_counts[(*category_index) as usize] = slot.count; // a position key will never be repeated across multiple slots, no need to add the counts together
        }

        // cleanup some unused portion of the Vec
        let mut last_used_index = 0;
        for i in 0..category_counts.len() {
            if category_counts[i] > 0 {
                last_used_index = i;
            }
        }
        let cat_counts = Vec::from(&category_counts[..=last_used_index]);
        return cat_counts;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub categories: HashSet<u32>,
    pub price: u32,
    pub projected_points: f64
}

impl Player {
    pub fn new(id: u64, name: &str, price: u32, projected_points: f64, categories: HashSet<u32>) -> Player {
        Player {
            id,
            name: String::from(name),
            categories,
            price,
            projected_points
        }
    }

    pub fn get_value(&self) -> f64{
        return (self.projected_points) / (self.price as f64);
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.projected_points.partial_cmp(&other.projected_points);
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) ->bool {
        return self.projected_points == other.projected_points;
    }
}

impl Eq for Player { }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::category_mapper::{ choose_category_mapper, NBAPositions };

    #[test]
    fn player_value() {
        let player = Player::new(1, "test player", 5000, 20.0, HashSet::new());
        assert_eq!(player.get_value(), 0.004);
    }

    #[test]
    fn context_category_counts() {
        let mut context = create_test_context();
        let category_counts = context.calculate_category_count(choose_category_mapper("nba").unwrap());
        let cpt_index = NBAPositions::Captain as usize;
        let util_index = NBAPositions::UTIL as usize;
        assert_eq!(category_counts[cpt_index], 1);
        assert_eq!(category_counts[util_index], 5);
    }

    fn create_test_context() -> LineupContext {
        let mut slots = Vec::new();
        slots.push(LineupSlot {
            name: String::from("captain"),
            count: 1,
            salary_multiplier: 1.5,
            point_multiplier: 1.5,
            key: String::from("CPT")
        });
        slots.push(LineupSlot {
            name: String::from("util"),
            count: 5,
            salary_multiplier: 1.0,
            point_multiplier: 1.0,
            key: String::from("UTIL")
        });
        let context = LineupContext {
            salary_cap: 50000,
            slots: slots,
            sport_type: SportType::nba,
        };
        
        context
    }
}