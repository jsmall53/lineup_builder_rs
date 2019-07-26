//public

//non-public mods
mod common;
mod category_mapper;
mod contest_reader;
mod slate_reader;
mod lineup_optimizer;

extern crate serde;
extern crate serde_json;

use std::cmp::Reverse;
use std::fs::File as STD_FILE;
use std::io::{ BufWriter, Write };
use common::{ LineupContext, LineupSlot, Player };
use contest_reader::ContestTemplateReader;
use category_mapper::CategoryMapper;
use lineup_optimizer::{ OptimizerContext, Optimizer };

pub struct LineupBuilder {
    resource_base_path: String,
    context: Option<LineupContext>,
    player_pool: Option<Vec<Player>>,
    dfs_provider: Option<String>,
    sport: Option<String>,
    contest_type: Option<String>,
    slate_path: Option<String>,
}

impl LineupBuilder {
    pub fn new(resource_path: &str) -> LineupBuilder {
        LineupBuilder {
            resource_base_path: String::from(resource_path),
            context: None,
            player_pool: None,
            dfs_provider: None,
            sport: None,
            contest_type: None,
            slate_path: None
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

    // set the contest 
    pub fn contest(mut self, contest: &str) -> Self {
        self.contest_type = Some(String::from(contest));
        self
    }

    /// Sets the path to the slate data
    pub fn slate(mut self, slate_path: &str) -> Self {
        self.slate_path = Some(String::from(slate_path));
        self
    }

    pub fn build(mut self) -> Self {
        let mut path = String::new();
        path.push_str(&self.resource_base_path);
        if let Some(p) = &self.dfs_provider {
            path.push_str(p);
            path.push('/');
        } else { // how am I handling errors here?

        }

        if let Some(s) = &self.sport {
            path.push_str(s);
            path.push('/');
        } else { // error

        }

        if let Some(c) = &self.contest_type {
            path.push_str(c);
            path.push_str(".json");
        }
        // read in the contest template from path
        self.context = Some(ContestTemplateReader::load(&path));

        // TODO: account for unimplemented lineup settings here
        //      i.e. 'salary_remaining', slotting players in to optimize around them, setting a distribution

        // choose the correct mapper
        if let Some(sport) = &self.sport {
            let mapper = category_mapper::choose_category_mapper(sport).unwrap();
            // read the slate to construct the player pool
            if let Some(slate_path) = &self.slate_path {
                let mut reader = slate_reader::SlateDataReader::new(slate_path);
                reader.read().unwrap();
                self.player_pool = Some(reader.get_player_pool(mapper));
                // for player in &self.player_pool {
                //     println!("{:?}", player);
                // }
            } else { // ERROR: no slate path

            }
            
        } else { // ERROR: unknown sport

        }

        self
    }

    pub fn optimize(&mut self) {
        let mapper = match &self.sport {
            Some(sport) => category_mapper::choose_category_mapper(sport).unwrap(),
            None => panic!("failed mapping categories for optimization")
        };

        let category_count = &self.context.as_mut().map(|c| c.calculate_category_count(mapper)).unwrap();

        // calculcate optimial lineup
        let optimizer_context = OptimizerContext::new(50000, category_count.clone(), self.player_pool.clone().unwrap());
        // println!("{:?}", &optimizer_context);
        let mut optimizer = Optimizer::new(optimizer_context);
        // fix this api lmao. what a messcargo 
        let optimizer_result = optimizer.optimize(self.player_pool.clone().unwrap().len() as u32, 50000, category_count.clone());
        // println!("{:?}", optimizer_result);
        let file: STD_FILE = STD_FILE::create("log.log").unwrap();
        let mut writer = BufWriter::new(&file);
        // for entry in optimizer.cache.iter() {
        //     write!(writer, "{:?}\n", entry).unwrap();
        // }
        // writer.flush();

        if optimizer_result.1 {
            // construct the lineup object from the result data
            let mut optimal_lineup: Vec<&Player> = Vec::new();
            
            for index in optimizer_result.2 {
                let player: &Player = self.player_pool.as_ref().unwrap().iter().nth(index).unwrap();
                optimal_lineup.push(player);
            }
            optimal_lineup.sort_by(|a,b| b.partial_cmp(a).unwrap());
            
            let mut point_total = 0.0;
            let mut salary_total = 0;
            let mut p = 1;
            println!("===Optimal Lineup===");
            for player in optimal_lineup {
                point_total += player.projected_points;
                salary_total += player.price;
                println!("{}.\t{:?}", p, player);
                p += 1;
            }
            println!("====================");
        } else { // no valid data
            println!("No valid data...");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lineup_builder_test() {
        println!("{:?}", std::env::current_dir());
        let mut builder = LineupBuilder::new("../resources/game_templates/")
                        .provider("draft_kings")
                        .sport("nba")
                        .contest("showdown")
                        .slate("../data/DKSalaries.csv")
                        .build();

        builder.optimize();
        assert!(false); // TODO: this is just to be able to see the output in the console until I setup the main application
    }
}