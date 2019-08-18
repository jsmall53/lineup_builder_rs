use std::collections::{ HashSet, HashMap };
use std::str::FromStr;

use lp_modeler;
use lp_modeler::problem::*;
use lp_modeler::problem::{LpObjective, Problem, LpProblem };
use lp_modeler::operations::{LpOperations};
use lp_modeler::variables::*;
use lp_modeler::variables::LpExpression::*;
use lp_modeler::solvers::{SolverTrait, CbcSolver};

use crate::common::{ BuilderState, Player, RosterSlot };
use crate::player_pool::{ PlayerPool };

pub struct LpOptimizer {
    player_pool: PlayerPool,
    problem: LpProblem,
    vars: HashMap<u64, LpBinary>
}

impl LpOptimizer {
    pub fn new(player_pool: PlayerPool) -> LpOptimizer {
        LpOptimizer {
            player_pool,
            problem: LpProblem::new("lp_optimizer", LpObjective::Maximize),
            vars: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, builder_state: &BuilderState, mapped_indices: &HashMap<String, u32>) {
        self.define_variables();
        self.define_objective_fn();
        let roster_slots: Vec<RosterSlot> = match &builder_state.roster_slots {
            Some(ref rs) => rs.clone(),
            None => panic!("Catastrophic error, no roster slots to fill!")
        };
        let salary_cap = &builder_state.salary_cap.unwrap();
        self.define_constaints(&roster_slots, *salary_cap, mapped_indices);
    }

    pub fn solve(&self) -> Result<Vec<u64>, String> {
        let solver = CbcSolver::new();
        // println!("{:?}", &self.problem);
        self.problem.write_lp("./debug_log/debug.lp");
        match solver.run(&self.problem) {
            Ok((_, var_values)) => {
                let mut player_ids: Vec<u64> = Vec::new();
                for (name, value) in var_values {
                    if value == 1.0 {
                        let id: u64 = match name[2..].parse::<u64>() {
                            Ok(id) => id,
                            Err(_) => {
                                std::u64::MAX
                            }
                        };
                        if id != std::u64::MAX {
                            player_ids.push(id);
                        } else {
                            return Err("failed to parse result id".to_string());
                        }
                    }
                }
                return Ok(player_ids);
            },
            Err(err) => {
                return Err(format!("Solver error: {}", err));
            }
        }
    }

    fn define_variables(&mut self) {
        for player in self.player_pool.get_all() {
            let var_name = format!("P_{}", player.id);
            self.vars.insert(player.id, LpBinary::new(&var_name));
        }
    }

    fn define_objective_fn(&mut self) {
        let mut obj_vec: Vec<LpExpression> = Vec::new();
        for (&id, var) in &self.vars {
            let obj_coef: Vec<f32> = self.player_pool.iter()
                    .filter(|(i,_)| **i == id).map(|(_,p)| p.projected_points as f32).take(1).collect();
            obj_vec.push(obj_coef[0] * var);
        }
        self.problem += lp_sum(&obj_vec);
    }

    fn define_constaints(&mut self, roster_slots: &[RosterSlot], salary_cap: u32, mapped_indices: &HashMap<String, u32>) {
        // Constraint 1: each position group must contain exactly N items (as specified by the constest template)
        // need: contest roster positions, position groups for each roster position
        for slot in roster_slots {
            let slot_count = slot.count;
            let group_id = mapped_indices.get(&slot.key).expect("optimizer failed to map group id");
            let group = self.player_pool.get_group(group_id);
            let mut group_constraint: Vec<LpExpression> = Vec::new();
            for player in group {
                let var = self.vars.get(&player.id).unwrap();
                group_constraint.push(1.0 * var);
            }
            // Into<LpExpression> is only implemented for f32 and i32 integer types. these values are represtative of real world positions on teams so they will be low enough that this conversion should never become an issue
            self.problem += lp_sum(&group_constraint).equal(slot_count as i32);
        }

        // // Constraint 2: each player may only be included once.
        // for (_, var) in &self.vars {
        //     self.problem += (1.0 * var).le(1);
        // }

        // Constraint 3: salaries of each player cannot exceed salary cap
        let mut cons_vec: Vec<LpExpression> = Vec::new();
        for (id, var) in &self.vars {
            let salary_coef: f32 = self.player_pool.get_player(id).unwrap().price as f32;
            cons_vec.push(salary_coef * var);
        }
        // Into<LpExpression>
        self.problem += lp_sum(&cons_vec).le(salary_cap as i32);
    }

    fn define_showdown_constraints(&mut self) {
        // TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lp_optimizer() {
        let players = get_test_players();
        let player_pool = PlayerPool::new(players, true);
        let builder_state = BuilderState {
            player_pool: Some(player_pool.clone()),
            player_data_list: None,
            roster_slots: Some(get_test_roster_slots()),
            salary_cap: Some(16300),
        };
        let mapped_indices = hashmap!{
            "QB".to_string() => 1,
            "RB".to_string() => 2,
            "WR".to_string() => 3,
        };

        let mut optimizer: LpOptimizer = LpOptimizer::new(player_pool.clone());
        optimizer.initialize(&builder_state, &mapped_indices);
        match optimizer.solve() {
            Ok(ids) => {
                for id in ids {
                    let player = player_pool.get_player(&id).unwrap();
                    println!("{:?}", player);
                }
                assert!(true);
            },
            Err(err) => {
                println!("{:?}", err);
                assert!(false);
            }
        }
    }

    // #[test]
    fn learning() {
        let ref a = LpInteger::new("a");
        let ref b = LpInteger::new("b");
        let ref c = LpInteger::new("c");

        // define problem and objective
        let mut problem = LpProblem::new("Test Problem", LpObjective::Maximize);
        // objective function
        problem += 10.0 * a + 20.0 * b;
        // define a constraint: 500a + 1200b + 1500c <= 10000
        problem += (500 * a + 1200 * b + 1500 * c).le(10000);
        //define a constraint: a <= b
        problem += (a).le(b);

        let solver = CbcSolver::new();

        match solver.run(&problem) {
            Ok((status, var_values)) => {
                println!("Status {:?}", status);
                for (name, value) in var_values.iter() {
                    println!("value of {} = {}", name, value);
                }
                assert!(true);
            },
            Err(msg) => {
                println!("TEST ERROR: {}", msg);
                assert!(false);
            },
        }
    }

    fn his() {
        // Define problem variables
        let ref a = LpInteger::new("a");
        let ref b = LpInteger::new("b");
        let ref c = LpInteger::new("c");

        // Define problem and objective sense
        let mut problem = LpProblem::new("test_problem", LpObjective::Maximize);

        // Objective Function: Maximize 10*a + 20*b
        problem += 10.0 * a + 20.0 * b;

        // Constraint: 500*a + 1200*b + 1500*c <= 10000
        problem += (500*a + 1200*b + 1500*c).le(10000);

        // Constraint: a <= b
        problem += (a).le(b);

        // Specify solver
        let solver = CbcSolver::new();

        // Run optimisation and process output hashmap
        match solver.run(&problem) {
            Ok((status, var_values)) => {
                println!("Status {:?}", status);
                for (name, value) in var_values.iter() {
                    println!("value of {} = {}", name, value);
                }
                assert!(true);
            },
            Err(msg) => {
                println!("{}", msg);
                assert!(false);
            },
        }
    }

    // #[test]
    fn test_matchmaking() {
        // Problem Data
        let men = vec!["A", "B", "C"];
        let women = vec!["D", "E", "F"];
        let compat_scores = hashmap!{
            ("A", "D") => 50.0,
            ("A", "E") => 75.0,
            ("A", "F") => 75.0,
            ("B", "D") => 60.0,
            ("B", "E") => 95.0,
            ("B", "F") => 80.0,
            ("C", "D") => 60.0,
            ("C", "E") => 70.0,
            ("C", "F") => 80.0,
        };

        // Define Problem
        let mut problem = LpProblem::new("Matchmaking", LpObjective::Maximize);

        // Define Variables
        let mut vars = HashMap::new();
        for m in &men{
            for w in &women{
                vars.insert((m, w), LpBinary::new(&format!("{}_{}", m, w)));
            }
        }

        // Define Objective Function
        let mut obj_vec: Vec<LpExpression> = Vec::new();
        for (&(&m, &w), var) in &vars{
            let obj_coef = compat_scores.get(&(m, w)).unwrap();
            obj_vec.push(*obj_coef * var);
        }
        problem += lp_sum(&obj_vec);

        // Define Constraints
        // Constraint 1: Each man must be assigned to exactly one woman
        for m in &men{
            let mut constr_vec = Vec::new();

            for w in &women{
                constr_vec.push(1.0 * vars.get(&(m, w)).unwrap());
            }

            problem += lp_sum(&constr_vec).equal(1);
        }

        // Constraint 2: Each woman must be assigned to exactly one man
        for w in &women{
            let mut constr_vec = Vec::new();

            for m in &men{
                constr_vec.push(1.0 * vars.get(&(m, w)).unwrap());
            }

            problem += lp_sum(&constr_vec).equal(1);
        }

        // Run Solver
        let solver = CbcSolver::new();
        let result = solver.run(&problem);

        // Terminate if error, or assign status & variable values
        assert!(result.is_ok(), result.unwrap_err());
        let (solver_status, var_values) = result.unwrap();

        // Compute final objective function value
        let mut obj_value = 0f32;
        for (&(&m, &w), var) in &vars{
            let obj_coef = compat_scores.get(&(m, w)).unwrap();
            let var_value = var_values.get(&var.name).unwrap();
            
            obj_value += obj_coef * var_value;
        }

        // Print output
        println!("Status: {:?}", solver_status);
        println!("Objective Value: {}", obj_value);
        // println!("{:?}", var_values);
        for (var_name, var_value) in &var_values{
            let int_var_value = *var_value as u32;
            if int_var_value == 1{
                println!("{} = {}", var_name, int_var_value);
            }
        }
    }

    // #[test]
    fn test_dfs_context() {
        // get the test data
        let players = get_test_players();
        
        let mut problem = LpProblem::new("test", LpObjective::Maximize);

        // define the variables
        let mut vars = HashMap::new();
        for player in &players {
            let var_name: String = format!("P_{}", player.id);
            vars.insert(&player.id, LpBinary::new(&var_name));
        }

        let mut obj_vec: Vec<LpExpression> = Vec::new();
        // define objective function
        for (&id, var) in &vars {
            let obj_coef: Vec<f32> = players.iter().filter(|p| &p.id == id).map(|p| p.projected_points as f32).take(1).collect();
            obj_vec.push(obj_coef[0] * var);
        }
        problem += lp_sum(&obj_vec);
        // define constraints
        
        for category in vec![1, 2, 3] {
            let group_players = players.iter().filter(|p| p.categories.contains(&category));
            let mut group_constraint: Vec<LpExpression> = Vec::new();
            for player in group_players {
                let var = vars.get(&player.id).unwrap();
                group_constraint.push(1.0 * var);
            }
            // println!("{:?}", group_constraint);
            problem += lp_sum(&group_constraint).equal(1); // just one from each category
        }

        // Constraint 2: each player may only occur once (not needed with single item categories, but going to include
        // for completness)
        for (&name, var) in &vars {
            let constraint = (1.0 * var).le(1);
            problem += constraint;
        }
        
        let solver = CbcSolver::new();

        // problem.write_lp("test_again.lp");
        match solver.run(&problem) {
            Ok((status, var_values)) => {
                let mut player_ids: Vec<u64> = Vec::new();
                for (name, value) in &var_values {
                    if value == &1.0 {
                        let id: u64 = match name[2..].parse::<u64>() {
                            Ok(id) => id,
                            Err(err) => {
                                println!("Error parsing player id: {:?}", err);
                                assert!(false);
                                69
                            }
                        };
                        if id != 69 {
                            player_ids.push(id);
                        } else {
                            assert!(false);
                        }
                    }
                }

                for p in players {
                    if player_ids.contains(&p.id) {
                        println!("{:?}", p);
                    }
                }
                assert!(true);
            },
            Err(err) => {
                println!("Solver error: {:?}", err);
                assert!(false);
            },
        }
    }

    fn get_test_players() -> Vec<Player> {
        let mut players = Vec::new();
        players.push(Player {
            id: 0,
            name: String::from("Tom Brady"),
            categories: hashset!{1},
            price: 4500,
            projected_points: 18.4,
        });
        players.push(Player {
            id: 1,
            name: String::from("Lamar Jackson"),
            categories: hashset!{1},
            price: 5200,
            projected_points: 24.8,
        });
        players.push(Player {
            id: 2,
            name: String::from("Todd Gurley"),
            categories: hashset!{2},
            price: 7000,
            projected_points: 21.2,
        });
        players.push(Player {
            id: 3,
            name: String::from("Alvin Kamara"),
            categories: hashset!{2},
            price: 6700,
            projected_points: 19.8,
        });
        players.push(Player {
            id: 4,
            name: String::from("Saquon Barkley"),
            categories: hashset!{2},
            price: 7300,
            projected_points: 26.0,
        });
        players.push(Player {
            id: 5,
            name: String::from("Desean Jackson"),
            categories: hashset!{3},
            price: 3700,
            projected_points: 10.4,
        });
        players.push(Player {
            id: 6,
            name: String::from("Deandre Hopkins"),
            categories: hashset!{3},
            price: 6800,
            projected_points: 19.9,
        });
        players.push(Player {
            id: 7,
            name: String::from("Davante Adams"),
            categories: hashset!{3},
            price: 6700,
            projected_points: 17.3,
        });
        players
    }

    fn get_test_roster_slots() -> Vec<RosterSlot> {
        let qb_slot = RosterSlot {
            name: "Quarterback".to_string(),
            key: "QB".to_string(),
            count: 1,
            salary_multiplier: 1.0,
            point_multiplier: 1.0,
        };

        let rb_slot = RosterSlot {
            name: "Runningback".to_string(),
            key: "RB".to_string(),
            count: 1,
            salary_multiplier: 1.0,
            point_multiplier: 1.0,
        };

        let wr_slot = RosterSlot {
            name: "Wide Receiver".to_string(),
            key: "WR".to_string(),
            count: 1,
            salary_multiplier: 1.0,
            point_multiplier: 1.0,
        };

        vec![qb_slot, rb_slot, wr_slot]
    }
}
