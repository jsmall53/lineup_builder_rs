use std::cell::{ RefCell };
use std::collections::{ BTreeMap, BTreeSet };
use std::rc::{ Rc };
use crate::common::{ Player };

#[derive(Debug)]
pub struct OptimizerContext {
    weight: u32,
    categories: Vec<u32>,
    items: Rc<Vec<Player>>,
}

impl OptimizerContext {
    pub fn new(weight: u32, categories: Vec<u32>, items: Rc<Vec<Player>>) -> OptimizerContext {
        OptimizerContext {
            weight,
            categories,
            items
        }
    }
}

type CacheKey = (u32, u32, Rc<RefCell<Vec<u32>>>);
type CacheValue = (f64, bool, BTreeSet<usize>);

pub struct Optimizer {
    pub cache: BTreeMap<CacheKey, CacheValue>,
    context: OptimizerContext,
    depth: u64,
}

impl Optimizer {
    pub fn new(context: OptimizerContext) -> Optimizer {
        Optimizer {
            cache: BTreeMap::new(),
            context,
            depth: 0,
        }
    }

    pub fn optimize(&mut self) -> CacheValue {
        let n = self.context.items.len() as u32;
        let categories = Rc::new(RefCell::new(self.context.categories.clone()));
        return self.optimize_impl(n, self.context.weight, categories);
    }

    fn optimize_impl(&mut self, n: u32, weight: u32, categories: Rc<RefCell<Vec<u32>>>) -> CacheValue {
        self.depth += 1;
        // print!("\rdepth: {}", self.depth);
        if let Some(it) = self.cache.get_mut(&(n, weight, categories.clone())) { // dont need to clone!
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
        for i in 0..categories.borrow().len() {
            if categories.borrow()[i] > category_count[i] {
                let key = (n, weight, Rc::new((*categories).clone())); // need to clone/copy!
                let value = (0.0, false, BTreeSet::<usize>::new());
                self.cache.insert(key.clone(), value);
                return self.cache.get(&key).unwrap().clone();
            }
        }

        let mut sum_categories = 0;
        for cat in categories.borrow().iter() {
            sum_categories += cat;
        }

        if n == 0 || sum_categories == 0 {
            let key = (n.clone(), weight, Rc::new((*categories).clone())); // need to clone/copy!
            let value = (0.0, true, BTreeSet::<usize>::new());
            self.cache.insert(key.clone(), value);
            return self.cache.get(&key).unwrap().clone();
        }

        let next_item: &Player = &self.context.items[(n as usize) - 1];
        let item_val = next_item.projected_points;
        let item_weight = next_item.price;

        let mut next_value: f64 = 0.0;
        let mut next_valid = true;
        let mut next_set: BTreeSet<usize> = BTreeSet::new(); // need to avoid this extra allocation somehow
        let category_list: Vec<usize> = next_item.categories.clone().into_iter().map(|c| c as usize).collect();
        for category in &category_list {
            if item_weight <= weight && categories.borrow()[*category] > 0 {
                let mut new_k_take = categories.clone(); // don't need to clone!
                new_k_take.borrow_mut()[*category] -= 1;
                let take = self.optimize_impl(n - 1, weight - item_weight, new_k_take);

                let mut new_k_reject = categories.clone(); // don't need to clone!
                new_k_reject.borrow_mut()[*category] += 1;
                let reject = self.optimize_impl(n - 1, weight, new_k_reject);
                
                let a = item_val + take.0;
                let b = reject.0;
                if take.1 && reject.1 { // if both paths were valid
                    if a > b {
                        next_value = a;
                        next_set = take.2;
                        let mut duplicate = false;
                        if !duplicate {
                            next_set.insert((n as usize) - 1);
                            break;
                        } else  {
                            let (n_value, n_valid, n_set) = self.optimize_impl(n - 1, weight, categories.clone()); // don't need to clone!
                            next_value = n_value;
                            next_valid = n_valid;
                            next_set = n_set;
                        }
                    } else { // a < b
                        next_set = reject.2;
                        next_value = b;
                    }
                } else if take.1 { // if only the take path is valid
                    next_set = take.2;
                    let mut duplicate = false;
                    if !duplicate {
                        next_set.insert((n as usize) - 1);
                        next_value = a;
                        break;
                    } else {
                        let (n_value, n_valid, n_set) = self.optimize_impl(n - 1, weight, categories.clone()); // don't need to clone!
                        next_value = n_value;
                        next_valid = n_valid;
                        next_set = n_set;
                    }
                } else if reject.1 { // if only the reject path is valid
                    next_value = b;
                    next_set = reject.2;
                } else { // no valid path
                    next_value = 0.0;
                    next_valid = false;
                    next_set = BTreeSet::new();
                }
            } else {
                let (n_value, n_valid, n_set) = self.optimize_impl(n - 1, weight, categories.clone()); // don't need to clone!
                next_value = n_value;
                next_valid = n_valid;
                next_set = n_set;
            }
        }

        let ret_value = (next_value, next_valid, next_set);
        let final_key = (n.clone(), weight, Rc::new((*categories).clone())); // need to clone/copy!
        self.cache.insert(final_key, ret_value.clone());
        self.depth -= 1;
        // print!("\rdepth: {}", self.depth);
        return ret_value;
    }
}

