// temporary
use std::collections::{ HashMap, HashSet, BTreeMap, BTreeSet };
use crate::common::{ Player };

#[derive(Debug)]
pub struct OptimizerContext {
    weight: u32,
    categories: Vec<u32>,
    items: Vec<Player>,
}

impl OptimizerContext {
    pub fn new(weight: u32, categories: Vec<u32>, items: Vec<Player>) -> OptimizerContext {
        OptimizerContext {
            weight,
            categories,
            items
        }
    }
}

type CacheKey = (u32, u32, Vec<u32>);
type CacheValue = (f64, bool, BTreeSet<usize>);

pub struct Optimizer {
    pub cache: BTreeMap<CacheKey, CacheValue>,
    context: OptimizerContext,
}

impl Optimizer {
    pub fn new(context: OptimizerContext) -> Optimizer {
        Optimizer {
            cache: BTreeMap::new(),
            context
        }
    }

    pub fn optimize(&mut self, n: u32, weight: u32, categories: Vec<u32>) -> CacheValue {
        return self.optimize_impl(n, weight, categories);
    }

    fn optimize_impl(&mut self, n: u32, weight: u32, categories: Vec<u32>) -> CacheValue {
        if let Some(it) = self.cache.get_mut(&(n, weight, categories.clone())) {
            // println!("ALREADY PROCESSED: {}, {}", n, weight);
            return it.clone();
        }

        // count categories for n number of items in the context
        let num_categories = self.context.categories.len();
        let mut category_count: Vec<u32> = vec![0; num_categories];
        for i in 0..n as usize {
            for cat in &(self.context.items[i].categories) {
                category_count[*(cat) as usize] += 1;
            }
        }

        // check the current state of the input categories against the category count that was just calculated
        for i in 0..categories.len() {
            // println!("item category count: {}, categories needed: {}", category_count[i], categories[i]);
            if categories[i] > category_count[i] {
                // println!("INVALID: not enough categories");
                let key = (n, weight, categories.clone());
                let value = (0.0, false, BTreeSet::<usize>::new());
                self.cache.insert(key.clone(), value);
                return self.cache.get(&key).unwrap().clone();
            }
        }

        let mut sum_categories = 0;
        for cat in &categories {
            sum_categories += cat;
        }

        if n == 0 || sum_categories == 0 {
            // println!("Invalid, n == 0 || sum_categories == 0");
            let key = (n.clone(), weight, categories.clone());
            let value = (0.0, true, BTreeSet::<usize>::new());
            self.cache.insert(key.clone(), value);
            return self.cache.get(&key).unwrap().clone();
        }

        let next_item: Player = self.context.items[(n as usize) - 1].clone();
        let item_val = next_item.projected_points;
        let item_weight = next_item.price;
        let category_list: Vec<usize> = next_item.categories.clone().into_iter().map(|c| c as usize).collect();
        let category = category_list[0];
        let current_name = next_item.name;
        let mut next_value: f64 = 0.0;
        let mut next_valid = true;
        let mut next_set: BTreeSet<usize>;
        if item_weight <= weight && categories[category] > 0 {
            let mut new_k_take = categories.clone();
            new_k_take[category] -= 1;
            let take = self.optimize(n - 1, weight - item_weight, new_k_take);

            let new_k_reject = categories.clone();
            // new_k_reject[category] += 1;
            let reject = self.optimize(n - 1, weight, new_k_reject);
            
            let a = item_val + take.0;
            let b = reject.0;
            if take.1 && reject.1 { // if both paths were valid
                // println!("take and reject both valid");
                if a > b {
                    next_value = a;
                    next_set = take.2;
                    let mut duplicate = false;
                    for i in &next_set {
                        if current_name == self.context.items[*i].name  {
                            duplicate = true;
                            // println!("DEBUG: duplicate player {}", current_name);
                        }
                    }
                    if !duplicate {
                        next_set.insert((n as usize) - 1);
                    } else  {
                        let (n_value, n_valid, n_set) = self.optimize(n - 1, weight, categories.clone());  
                        next_value = n_value;
                        next_valid = n_valid;
                        next_set = n_set;
                    }
                } else { // a < b
                    next_set = reject.2;
                    next_value = b;
                }
            } else if take.1 { // if only the take path is valid
                // println!("reject invalid");
                next_set = take.2;
                let mut duplicate = false;
                for i in &next_set {
                    if current_name == self.context.items[*i].name  {
                        duplicate = true;
                        // println!("DEBUG: duplicate player {}", current_name);
                    }
                }
                if !duplicate {
                    next_set.insert((n as usize) - 1);
                    next_value = a;
                } else {
                    let (n_value, n_valid, n_set) = self.optimize(n - 1, weight, categories.clone());
                    next_value = n_value;
                    next_valid = n_valid;
                    next_set = n_set;
                }
            } else if reject.1 { // if only the reject path is valid
                // println!("take invalid");
                next_value = b;
                next_set = reject.2;
            } else { // no valid path
                // println!("take and reject both invalid");
                next_value = 0.0;
                next_valid = false;
                next_set = BTreeSet::new();
            }
        } else {
            // println!("item weight or category count incorrect");
            let (n_value, n_valid, n_set) = self.optimize(n - 1, weight, categories.clone());  
            next_value = n_value;
            next_valid = n_valid;
            next_set = n_set;
        }

        let ret_value = (next_value, next_valid, next_set);
        let final_key = (n.clone(), weight, categories.clone());
        self.cache.insert(final_key, ret_value.clone());
        // println!("{:?}", ret_value);
        return ret_value;
    }
}

