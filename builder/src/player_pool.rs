use std::collections::{ HashMap };
use std::collections::hash_map::Iter;
use crate::common::{ Player };

#[derive(Debug, Clone, Default)]
pub struct PlayerPool {
    player_map: HashMap<u64, Player>,
}

// TODO: contains a lot of cloning, figure out a better model to avoid excessive memory use
impl PlayerPool {
    pub fn new(mut players: Vec<Player>, optimize: bool) -> PlayerPool {
        if optimize {
            players.retain(|p| p.projected_points > 3.0);
        }
        players.sort_by(|a,b| b.partial_cmp(a).unwrap());

        let mut player_map = HashMap::new();
        for player in &players {
            player_map.insert(player.id, player.clone()); // TODO: FIX THIS CLONE, NEED ONE SOURCE OF TRUTH
        }

        PlayerPool {
            player_map,
        }
    }

    pub fn get_player(&self, player_id: &u64) -> Option<&Player> {
        self.player_map.get(player_id)
    }

    /// Returns a list of all players currently in the pool
    pub fn get_all(&self) -> Vec<Player> {
        let mut players: Vec<Player> = self.player_map.iter().map(|(_,v)| v.clone()).collect();
        players.sort_by(|a,b| b.partial_cmp(a).unwrap());
        players
    }

    /// Gets a list of all players the belong to the given group
    pub fn get_group(&self, group_id: &u32) -> Vec<Player> {
        let mut list: Vec<Player> = self.player_map.iter()
            .filter(|(_,p)| p.categories.contains(group_id))
            .map(|(_,p)| p.clone())
            .collect();
        list.sort_by(|a,b| b.partial_cmp(a).unwrap());
        list
    }

    /// Excludes a single player from the list of players
    pub fn exclude_player(&self, player_id: u64) -> Vec<Player> {
        let mut list: Vec<Player> = self.player_map.iter()
            .filter(|(_,p)| p.id != player_id)
            .map(|(_,p)| p.clone())
            .collect();
        list.sort_by(|a,b| b.partial_cmp(a).unwrap());
        list
    }

    /// Excludes a list of players
    pub fn exclude_players(&self, player_ids: Vec<u64>) -> Vec<Player> {
        let mut list: Vec<Player> = self.player_map.iter()
            .filter(|(k,_)| !player_ids.contains(&k))
            .map(|(_,p)| p.clone())
            .collect();
        list.sort_by(|a,b| b.partial_cmp(a).unwrap());
        list
    }

    /// Excludes an entire group of players from the list of players.
    /// `group_id` is the numeric value of the player group
    pub fn exclude_group(&self, group_id: u32) -> Vec<Player> {
        let mut list: Vec<Player> = self.player_map.iter()
            .filter(|(_,p)| !p.categories.contains(&group_id))
            .map(|(_,p)| p.clone())
            .collect();
        list.sort_by(|a,b| b.partial_cmp(a).unwrap());
        list
    }

    pub fn iter(&self) -> Iter<u64, Player> {
        // let list: Vec<Player> = self.player_map.iter()
        //     .map(|(_,p)| p.clone())
        //     .collect();
        // list.iter()
        self.player_map.iter()
    }
}
