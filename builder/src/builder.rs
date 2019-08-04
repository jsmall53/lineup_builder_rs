use std::fs::{ File as STD_FILE };
use std::io::{ BufWriter, Write };
use std::rc::{ Rc };
use std::cell::{ Cell, RefCell };

use crate::category_mapper;
use crate::common;
use crate::common::{ BuilderState, Player };
use crate::contest_reader::{ load_contest };
use crate::slate_reader::{ read_slate };
use crate::lineup_optimizer::{ OptimizerContext, Optimizer };

pub struct Lineup {
    player_list: Vec<Player>,
}

impl Lineup {
    pub fn new(player_list: Vec<Player>) -> Lineup {
        Lineup {
            player_list
        }
    }

    pub fn expected_result(&self) -> (f64, u32) {
        let mut point_total = 0.0;
        let mut salary_total = 0;
        for player in &self.player_list {
            point_total += player.projected_points;
            salary_total += player.price;
        }
        (point_total, salary_total)
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        let mut p = 1;
        for player in &self.player_list {
            output.push_str(&format!("{}. {:?}\n", p, player));
            p += 1;
        }
        output.push_str("--------------------------\n");
        let (point_total, salary_total) = self.expected_result();
        output.push_str(&format!("Point Total: {}, Salary Total: {}", point_total, salary_total));
        output
    }
}

pub struct Builder {
    resource_path: String,
    dfs_provider: Option<String>,
    sport: Option<String>,
    contest_type: Option<String>,
    slate_path: Option<String>,
    builder_state: Option<BuilderState>,
}

impl Builder {
    pub fn new(resource_path: &str) -> Builder {
        Builder {
            resource_path: String::from(resource_path),
            dfs_provider: None,
            sport: None,
            contest_type: None,
            slate_path: None,
            builder_state: None,
        }
    }

    pub fn provider(mut self, provider: &str) -> Self {
        self.dfs_provider = Some(String::from(provider));
        self
    }

    pub fn sport(mut self, sport: &str) -> Self {
        self.sport = Some(String::from(sport));
        self
    }

    pub fn contest(mut self, contest: &str) -> Self {
        self.contest_type = Some(String::from(contest));
        self
    }

    pub fn slate(mut self, slate_path: &str) -> Self {
        self.slate_path = Some(String::from(slate_path));
        self
    }

    pub fn build(mut self) -> Result<Self, &'static str> {
        let mut path = String::new();
        path.push_str(&self.resource_path);
        if !&self.resource_path.ends_with('/') { path.push('/') };
        let provider = match &self.dfs_provider {
            Some(ref p) => p,
            None => return Err("no dfs provider specified"),
        };
        path.push_str(provider);
        path.push('/');
        
        if let Some(s) = &self.sport {
            path.push_str(s);
            path.push('/');
        } else { // error
            return Err("no sport specified");
        }

        if let Some(c) = &self.contest_type {
            path.push_str(c);
            path.push_str(".json");
        } else {
            return Err("no contest type specified");
        }

        let mut builder_state = BuilderState {
            player_data_list: None,
            roster_slots: None,
            salary_cap: None,
        };

        // TODO: account for unimplemented lineup settings here
        //      i.e. 'salary_remaining', slotting players in to optimize around them, setting a distribution

        // choose the correct mapper
        if let Some(sport) = &self.sport {
            let mapped_indices = category_mapper::map_categories(sport).unwrap();
            load_contest(&path, &mut builder_state);
            if let Some(slate_path) = &self.slate_path {
                read_slate(slate_path, provider, &mut builder_state, &mapped_indices)?;
            } else { // ERROR: no slate path
                return Err("no slate path specified");
            }
        } else { // ERROR: unknown sport
            return Err("no sport specified");
        }
        self.builder_state = Some(builder_state);
        Ok(self)
    }

    pub fn optimize(&self) -> Result<Vec<Lineup>, &'static str> {
        let mapped_indices = match &self.sport {
            Some(sport) => category_mapper::map_categories(sport).unwrap(),
            None => panic!("failed mapping categories for optimization")
        };

        match &self.builder_state {
            Some(ref s) => {
                let category_count = common::calculate_category_count(s, &mapped_indices);
                let player_pool: Vec<Player> = s.player_data_list.clone().unwrap()
                    .iter()
                    .filter(|p| p.projected_points > 0.0)
                    .map(|p| p.clone())
                    .collect();;
                let optimized_player_pool: Rc<Vec<Player>> = Rc::new(player_pool);
                let salary_cap = &s.salary_cap;
                
                // calculcate optimial lineup
                let optimizer_context = OptimizerContext::new(salary_cap.clone().unwrap(), category_count.clone(), Rc::clone(&optimized_player_pool));
                let mut optimizer = Optimizer::new(optimizer_context);
                let optimizer_result_best = optimizer.optimize();
                println!("optimizer_result_best: {:?}", optimizer_result_best);
                let take = 3;
                let mut sorted_results = Vec::new();
                let mut lineups = Vec::new();
                for entry in optimizer.cache.iter() {
                    let (_, optimizer_result) = entry;
                    if optimizer_result.1 {
                        sorted_results.push(optimizer_result);
                    }
                }
                sorted_results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                println!("sorted results len: {}", sorted_results.len());
                for result in sorted_results {
                    let mut optimal_lineup: Vec<Player> = Vec::new();
                    
                    for index in &result.2 {
                        let player: Player = optimized_player_pool.iter().nth(*index).map(|p| p.clone()).unwrap();
                        optimal_lineup.push(player);
                    }
                    optimal_lineup.sort_by(|a,b| b.partial_cmp(a).unwrap());
                    let lineup = Lineup::new(optimal_lineup);
                    lineups.push(lineup);
                    if lineups.len() >= take {
                        break;
                    }
                }
                if lineups.len() == 0 {
                     return Err("No valid data...");
                }
                return Ok(lineups);
            },
            None => panic!("Catastrophic error, no state available to get roster categories, please retry"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refcell_copying() {
        let mut ref_cell: RefCell<Vec<u32>> = RefCell::new(vec![2, 1, 1, 1, 1, 1, 3]);
        let orig_clone = ref_cell.clone();
        (*ref_cell.get_mut())[0] -= 1;
        assert_eq!(ref_cell, RefCell::new(vec![1, 1, 1, 1, 1, 1, 3]));
        assert_eq!(orig_clone, RefCell::new(vec![2, 1, 1, 1, 1, 1, 3]));
    }

    #[test]
    fn test_rc_refcell_clone() {
        let mut categories = Rc::new(RefCell::new(vec![2, 1, 1, 1, 1, 1, 3]));
        let orig_clone = Rc::new((*categories).clone());
        (*categories.borrow_mut())[0] -= 1;

        assert_eq!(categories, Rc::new(RefCell::new(vec![1, 1, 1, 1, 1, 1, 3])));
        assert_eq!(*orig_clone, RefCell::new(vec![2, 1, 1, 1, 1, 1, 3]));
    }
}