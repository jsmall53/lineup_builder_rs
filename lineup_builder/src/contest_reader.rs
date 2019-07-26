use std::fs;
use std::fs::{ File };
use std::io::{ Read, BufReader };
use serde_json;
use crate::common::{ LineupContext };


pub struct ContestTemplateReader { }

// TODO: implement save so I can write new contests out, (not by hand)
impl ContestTemplateReader {
    pub fn load(file_path: &str) -> LineupContext {
        let file = File::open(file_path)
                        .expect(&format!("failed to open contest template file: {}", file_path));
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        let size: usize = match buf_reader.read_to_string(&mut contents) {
            Ok(size) => size,
            Err(_err) => 0
        };
        let context: LineupContext = serde_json::from_str(&contents)
                    .expect("failed to parse contest template file");
        context
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_load() {
        println!("{:?}", env::current_dir());
        let context = ContestTemplateReader::load("../resources/game_templates/draft_kings/nba/classic.json");
        assert_eq!(context.salary_cap, 50000);
        assert_eq!(context.slots.len(), 8);
    }
}