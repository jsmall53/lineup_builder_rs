pub mod builder;

mod category_mapper;
mod common;
mod contest_reader;
mod slate_reader;
mod lineup_optimizer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
