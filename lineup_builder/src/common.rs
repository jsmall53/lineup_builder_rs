use std::cmp::{ Ordering };
use std::collections::{ HashSet };
use serde::{ Deserialize, Serialize };

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
    name: String,
    key: String,
    count: u32,
    salary_multiplier: f64,
    point_multiplier: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LineupContext {
    pub sport_type: SportType,
    pub slots: Vec<LineupSlot>,
    pub salary_cap: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Player {
    name: String,
    categories: HashSet<usize>,
    price: u32,
    projected_points: f64
}

impl Player {
    pub fn new(name: &str, price: u32, projected_points: f64, categories: HashSet<usize>) -> Player {
        Player {
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

    #[test]
    fn value() {
        let player = Player::new("test player", 5000, 20.0, HashSet::new());
        assert_eq!(player.get_value(), 0.004);
    }
}