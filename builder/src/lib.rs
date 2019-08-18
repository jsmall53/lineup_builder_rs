#![allow(dead_code)]
#![allow(unused_imports)]
#[macro_use]
extern crate lp_modeler;
#[macro_use]
extern crate serde;
#[macro_use] extern crate maplit;

pub mod builder;

mod category_mapper;
mod common;
mod contest_reader;
mod slate_reader;
mod lineup_optimizer;
mod lp_optimizer;
mod player_pool;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
