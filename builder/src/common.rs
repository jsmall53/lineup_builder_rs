use std::cmp::{ Ordering };
use std::collections::{ HashSet };
use serde::{ Deserialize, Serialize };
use crate::category_mapper::CategoryMapper;

#[derive(Debug, Deserialize, Serialize)]
pub struct RosterSlot {
    // id: i32,
    pub name: String,
    pub key: String,
    pub count: u32,
    pub salary_multiplier: f64,
    pub point_multiplier: f64,
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

pub struct BuilderState {
    pub player_data_list: Option<Vec<Player>>,
    pub roster_slots: Option<Vec<RosterSlot>>,
    pub salary_cap: Option<u32>,
}

pub fn calculate_category_count(builder_state: &BuilderState, mapper: impl CategoryMapper) -> Vec<u32> {
    let mut category_counts: Vec<u32> = vec!(0; 100);
    if let Some(ref slots) = &builder_state.roster_slots {
        for slot in slots {
            let index_map = mapper.map(&slot.key);
            let category_index = index_map.iter().take(1).next().unwrap(); // a slot will only ever have one position key
            category_counts[(*category_index) as usize] = slot.count; // a position key will never be repeated across multiple slots, no need to add the counts together
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_value() {
        let player = Player { 
            id: 1,
            name: String::from("test player"),
            price: 5000,
            projected_points: 20.0,
            categories: HashSet::new()
        };
        assert_eq!(player.get_value(), 0.004);
    }

}