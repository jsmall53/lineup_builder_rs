use std::fs::{ File as STD_FILE };
use std::io::{ BufWriter, Write };

use crate::category_mapper;
use crate::common;
use crate::common::{ BuilderState, Player };
use crate::contest_reader::{ load_contest };
use crate::slate_reader::{ read_slate };
use crate::lineup_optimizer::{ OptimizerContext, Optimizer };

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

    pub fn build(mut self) -> Self {
        let mut path = String::new();
        path.push_str(&self.resource_path);
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

        let mut builder_state = BuilderState {
            player_data_list: None,
            roster_slots: None,
            salary_cap: None,
        };

        // TODO: account for unimplemented lineup settings here
        //      i.e. 'salary_remaining', slotting players in to optimize around them, setting a distribution

        // choose the correct mapper
        if let Some(sport) = &self.sport {
            let mapper = category_mapper::choose_category_mapper(sport).unwrap();
            load_contest(&path, &mut builder_state);
            // read the slate to construct the player pool
            if let Some(slate_path) = &self.slate_path {
                // TODO: handle this error
                read_slate(slate_path, &mut builder_state, mapper);
            } else { // ERROR: no slate path

            }
            
        } else { // ERROR: unknown sport

        }
        self.builder_state = Some(builder_state);
        self
    }

    pub fn optimize(&self) {
        let mapper = match &self.sport {
            Some(sport) => category_mapper::choose_category_mapper(sport).unwrap(),
            None => panic!("failed mapping categories for optimization")
        };

        // let category_count = &self.context.as_mut().map(|c| c.calculate_category_count(mapper)).unwrap();
        let category_count = match &self.builder_state {
            Some(ref s) => common::calculate_category_count(s, mapper),
            None => panic!("Catastrophic error, no state available to get roster categories, please retry"),
        };

        let player_pool = match &self.builder_state {
            Some(ref s) => &s.player_data_list,
            None => panic!("Catastrophic error, no state available to get player pool. check inputs and retry"),
        };
        let salary_cap = match &self.builder_state {
            Some(ref s) => &s.salary_cap,
            None => panic!("Catatropic error, no state available to get salary cap. check inputs and retry"),
        };
        // calculcate optimial lineup
        let optimizer_context = OptimizerContext::new(salary_cap.clone().unwrap(), category_count.clone(), player_pool.clone().unwrap());
        let mut optimizer = Optimizer::new(optimizer_context);
        let optimizer_result = optimizer.optimize();
        let file: STD_FILE = STD_FILE::create("log.log").unwrap();
        let mut writer = BufWriter::new(&file);
        // for entry in optimizer.cache.iter() {
        //     write!(writer, "{:?}\n", entry).unwrap();
        // }
        // writer.flush();

        if optimizer_result.1 {
            // TODO: construct the lineup object from the result data
            let mut optimal_lineup: Vec<&Player> = Vec::new();
            
            for index in optimizer_result.2 {
                let player: &Player = player_pool.as_ref().unwrap().iter().nth(index).unwrap();
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