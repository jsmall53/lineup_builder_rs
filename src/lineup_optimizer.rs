// #[macro_use]
// use csv;
// use serde;
// #[macro_use]
// use serde_derive;

// use std::collections::{ HashMap, HashSet };
// use std::error::Error;
// use std::ops::Add;

// #[derive(Debug, Clone)]
// pub struct Player<T: Clone> {
//     name: String,
//     categories: HashSet<usize>,
//     price: u32,
//     projected_points: T,
// }

// impl<T: Clone> Player<T> {
//     pub fn new(name: &str, price: u32, projected_points: T, categories: HashSet<usize>) -> Player<T> {
//         Player {
//             name: String::from(name),
//             categories,
//             price,
//             projected_points
//         }
//     }
// }

// // TODO: do I have to force a specific form of the data?
// //       how can I normalize
// #[derive(Debug, Deserialize)]
// pub struct SlateDataRow {
//     #[serde(rename = "Position")]
//     position: String,
//     #[serde(rename = "Name + ID")]
//     name_id: String,
//     #[serde(rename = "Name")]
//     name: String,
//     #[serde(rename = "ID")]
//     id: i64,
//     #[serde(rename = "Roster Position")]
//     roster_position: String,
//     #[serde(rename = "Salary")]
//     salary: u32,
//     #[serde(rename = "Game Info")]
//     game_info: String,
//     #[serde(rename = "TeamAbbrev")]
//     teamabbrev: String,
//     #[serde(rename = "AvgPointsPerGame")]
//     avg_points_per_game: f64,
// }

// pub struct SlateDataReader {
//     player_data_list: Vec<SlateDataRow>,
//     file_name: String,
// }

// // type Record = HashMap<String, String>;
// // type Record2 = (String, String, String, i64, String, i64, String, String, String);

// impl SlateDataReader {
//     pub fn new(path: &str) -> SlateDataReader {
//         SlateDataReader {
//             player_data_list: Vec::new(),
//             file_name: String::from(path)
//         }
//     }

//     pub fn read(&mut self) -> Result<(), Box<Error>> {
//         let mut reader = csv::Reader::from_path(&self.file_name).unwrap();
//         for result in reader.deserialize() {
//             let record: SlateDataRow = result?;
//             self.player_data_list.push(record);
//         }
//         Ok(())
//     }

//     pub fn get_player_pool(&self) -> Vec<Player<f64>> {
//         let mut player_pool = Vec::new();
//         for row in &self.player_data_list {
//             let player = Player::new(&row.name, row.salary, row.avg_points_per_game, HashSet::new());
//             player_pool.push(player);
//         }
//         return player_pool;
//     }
// }

// pub struct OptimizerContext {
//     weight: u32,
//     categories: Vec<u32>,
//     items: Vec<Player<f64>>,
// }

// type CacheKey = (u32, u32, Vec<u32>);
// type CacheValue = (f64, bool, HashSet<usize>);

// struct Optimizer {
//     cache: HashMap<CacheKey, CacheValue>,
//     context: OptimizerContext,
// }

// impl Optimizer {
//     pub fn new(context: OptimizerContext) -> Optimizer {
//         Optimizer {
//             cache: HashMap::new(),
//             context
//         }
//     }

//     pub fn optimize(&mut self, n: u32, weight: u32, mut categories: Vec<u32>) -> CacheValue {
//         if let Some(it) = self.cache.get_mut(&(n, weight, categories.clone())) {
//             return it.clone();
//         }

//         // count categories for n number of items in the context
//         let num_categories = self.context.categories.len();
//         let mut category_count: Vec<u32> = vec![0; num_categories];
//         let c = n as usize;
//         for i in 0..c {
//             for cat in &(self.context.items[i].categories) {
//                 category_count[*cat] += 1;
//             }
//         }

//         // check the current state of the input categories against the category count that was just calculated
//         for i in 0..categories.len() {
//             if categories[i] > category_count[i] {
//                 let key = (n, weight, categories.clone());
//                 let value = (0.0, false, HashSet::<usize>::new());
//                 self.cache.insert(key.clone(), value);
//                 return self.cache.get(&key).unwrap().clone();
//             }
//         }

//         let mut sum_categories = 0;
//         for cat in &categories {
//             sum_categories += cat;
//         }

//         if n == 0 || sum_categories == 0 {
//             let key = (n.clone(), weight, categories.clone());
//             let value = (0.0, false, HashSet::<usize>::new());
//             self.cache.insert(key.clone(), value);
//             return self.cache.get(&key).unwrap().clone();
//         }

//         let next_item = self.context.items[(n as usize) - 1].clone();
//         let item_val = next_item.projected_points;
//         let item_weight = next_item.price;
//         let category_list: Vec<usize> = next_item.categories.clone().into_iter().collect();
//         let category = category_list[0];
//         let mut next_value: f64 = 0.0;
//         let mut next_valid = false;
//         let mut next_set: HashSet<usize> = HashSet::new();
//         if item_weight <= weight && categories[category] > 0 {
//             let mut new_k_take = categories.clone();
//             new_k_take[category] -= 1;
//             let take = self.optimize(n - 1, weight - item_weight, new_k_take);

//             let mut new_k_reject = categories.clone();
//             new_k_reject[category] += 1;
//             let reject = self.optimize(n - 1, weight, new_k_reject);
            
//             let a = item_val + take.0;
//             let b = reject.0;
//             if take.1 && reject.1 { // if both paths were valid
//                 if a > b {
//                     next_value = a;
//                     next_set = take.2;
//                     let duplicate = next_set.insert(n as usize - 1);
//                     if duplicate {
//                         let (n_value, n_valid, n_set) = self.optimize(n - 1, weight, categories.clone());  
//                         next_value = n_value;
//                         next_valid = n_valid;
//                         next_set = n_set;
//                     }
//                 } else { // a < b
//                     next_set = reject.2;
//                 }
//             } else if take.1 { // if only the take path is valid
//                 next_set = take.2;
//                 let duplicate = next_set.insert(n as usize - 1);
//                 if !duplicate {
//                     next_value = b;
//                 } else {
//                     let (n_value, n_valid, n_set) = self.optimize(n - 1, weight, categories.clone());
//                     next_value = n_value;
//                     next_valid = n_valid;
//                     next_set = n_set;
//                 }
//             } else if reject.1 { // if only the reject path is valid
//                 next_value = b;
//                 next_set = reject.2;
//             } else { // no valid path
//                 // TODO:
//                 next_value = 0.0;
//                 next_valid = false;
//                 next_set = HashSet::new();
//             }
//         } else {
//             let (n_value, n_valid, n_set) = self.optimize(n - 1, weight, categories.clone());  
//             next_value = n_value;
//             next_valid = n_valid;
//             next_set = n_set;
//         }
//         let ret_value = (next_value, next_valid, next_set);
//         let final_key = (n.clone(), weight, categories.clone());
//         self.cache.insert(final_key, ret_value.clone());
//         return ret_value;
//     }
// }
