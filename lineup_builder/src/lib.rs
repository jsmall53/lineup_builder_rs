//public

//non-public mods
mod common;
mod category_mapper;

use common::{ LineupContext, LineupSlot, Player };

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

        // setup the LineupContext

        // choose the correct mapper

        // read the slate to construct the player pool

        self
    }

    pub fn optimize(&self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lineup_builder_test() {
        let builder = LineupBuilder::new("test_data")
                        .provider("Draft Kings")
                        .sport("nba")
                        .contest("classic")
                        .slate("data.csv")
                        .build();

        builder.optimize();
        assert!(true);
    }
}