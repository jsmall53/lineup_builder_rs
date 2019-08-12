use crate::common::{ Player };

pub struct PlayerPool {
    players: Vec<Player>,
}

// TODO: contains a lot of cloning, figure out a better model to avoid excessive memory use
impl PlayerPool {
    pub fn new(mut players: Vec<Player>, optimize: bool) -> PlayerPool {
        if optimize {
            players.retain(|p| p.projected_points > 0.0);
        }
        players.sort_by(|a,b| b.partial_cmp(a).unwrap());
        PlayerPool {
            players
        }
    }

    /// Returns a list of all players currently in the pool
    pub fn get_all(&self) -> Vec<Player> {
        self.players.clone()
    }

    /// Gets a list of all players the belong to the given group
    pub fn get_group(&self, group_id: u32) -> Vec<Player> {
        self.players.iter()
            .filter(|p| p.categories.contains(&group_id))
            .map(|p| p.clone())
            .collect()
    }

    /// Excludes a single player from the list of players
    pub fn exclude_player(&self, player_id: u64) -> Vec<Player> {
        self.players.iter()
            .filter(|p| p.id != player_id)
            .map(|p| p.clone())
            .collect()
    }

    /// Excludes a list of players
    pub fn exclude_players(&self, player_ids: Vec<u64>) -> Vec<Player> {
        self.players.iter()
            .filter(|p| !player_ids.contains(&p.id))
            .map(|p| p.clone())
            .collect()
    }

    /// Excludes an entire group of players from the list of players.
    /// `group_id` is the numeric value of the player group
    pub fn exclude_group(&self, group_id: u32) -> Vec<Player> {
        self.players.iter()
            .filter(|p| !p.categories.contains(&group_id))
            .map(|p| p.clone())
            .collect()
    }
}