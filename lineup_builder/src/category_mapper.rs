use std::collections::{ HashSet };

pub trait CategoryMapper {
    fn map(&self, string: &str) -> HashSet<u32>;
}

pub fn choose_category_mapper(sport: &str) -> Option<impl CategoryMapper> {
    if sport == "nba" {
        return Some(NBACategoryMapper { });
    }

    None
}

/// these are DFS positions for the NBA (at least on draft kings...)
pub enum NBAPositions {
    PointGuard = 0,
	ShootingGuard = 1,
	SmallForward = 2,
	PowerForward = 3,
	Center = 4,
	Guard = 5,
	Forward = 6,
	UTIL = 7,
	Captain = 8,
}

struct NBACategoryMapper { }

impl NBACategoryMapper {
    fn get_pos_value(string: &str) -> u32 {
        if string == "PG" {
            NBAPositions::PointGuard as u32
        } else if string == "SG" {
            NBAPositions::ShootingGuard as u32
        } else if string == "SF" {
            NBAPositions::SmallForward as u32
        } else if string == "PF" {
            NBAPositions::PowerForward as u32
        } else if string == "C" {
            NBAPositions::Center as u32
        } else if string == "G" {
            NBAPositions::Guard as u32
        } else if string == "F" {
            NBAPositions::Forward as u32
        } else if string == "UTIL" {
            NBAPositions::UTIL as u32
        } else if string == "CPT" {
            NBAPositions::Captain as u32
        } else {
            std::u32::MAX
        }
    }
}

impl CategoryMapper for NBACategoryMapper {
    fn map(&self, string: &str) -> HashSet<u32> {
        let mut categories: HashSet<u32> = HashSet::new();
        let tokenized: Vec<&str> = string.split('/').collect();
        for token in tokenized {
            let pos_value = NBACategoryMapper::get_pos_value(token);
            categories.insert(pos_value);
        }
        return categories;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_each_nba() {
        let mut pos = "PG";
        let mapper = NBACategoryMapper {};
        let mut cats = mapper.map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::PointGuard as u32)), "should contain item mapping to point guard");

        pos = "SG";
        cats = mapper.map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::ShootingGuard as u32)), "should contain item mapping to shooting guard");
        
        pos = "SF";
        cats = mapper.map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::SmallForward as u32)), "should contain item mapping to small forward");

        pos = "PF";
        cats = mapper.map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::PowerForward as u32)), "should contain item mapping to power forward");

        pos = "C";
        cats = mapper.map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::Center as u32)), "should contain item mapping to center");

        pos = "G";
        cats = mapper.map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::Guard as u32)), "should contain item mapping to guard");

        pos = "F";
        cats = mapper.map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::Forward as u32)), "should contain item mapping to foward");

        pos = "UTIL";
        cats = mapper.map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::UTIL as u32)), "should contain item mapping to UTIL");

        pos = "CPT";
        cats = mapper.map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::Captain as u32)), "should contain item mapping to captain");
    }

    #[test]
    fn multiple_unique_nba() {
        let mut pos = "PG/G/UTIL/CPT";
        let mapper = NBACategoryMapper {};
        let cats = mapper.map(pos);
        assert!(cats.len() == 4);
        assert!(cats.contains(&(NBAPositions::PointGuard as u32)));
        assert!(cats.contains(&(NBAPositions::Guard as u32)));
        assert!(cats.contains(&(NBAPositions::UTIL as u32)));
        assert!(cats.contains(&(NBAPositions::Captain as u32)));
    }

    #[test]
    fn multiple_repeat_nba() {
        let mut pos = "PG/PG/UTIL";
        let mapper = NBACategoryMapper {};
        let cats = mapper.map(pos);
        assert!(cats.len() == 2);
        assert!(cats.contains(&(NBAPositions::PointGuard as u32)));
        assert!(cats.contains(&(NBAPositions::UTIL as u32)));
    }
}
