use std::collections::{ HashMap, HashSet };

trait CategoryMapper {
    fn get_all_categories() -> Vec<String>;
    fn map() -> HashMap<String, u32> {
        let mut categories: HashMap<String, u32> = HashMap::new();
        let cats = Self::get_all_categories();
        let mut i = 0;
        for cat in cats {
            categories.insert(cat, i);
            i += 1;
        }
        categories
    }
}

// TODO: integrate new mapper
pub fn map_categories(sport: &str) -> Option<HashMap<String, u32>> {
    if sport.to_lowercase() == "nba" {
        return Some(NBACategoryMapper::map());
    } else if sport.to_lowercase() == "mlb" {
        return Some(MLBCategoryMapper::map());
    } else if sport.to_lowercase() == "nfl" {
        return Some(NFLCategoryMapper::map());
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

impl CategoryMapper for NBACategoryMapper {
    fn get_all_categories() -> Vec<String> {
        vec![
            String::from("PG"),
            String::from("SG"),
            String::from("SF"),
            String::from("PF"),
            String::from("C"),
            String::from("G"),
            String::from("F"),
            String::from("UTIL"),
            String::from("CPT"),
        ]
    }
}

struct MLBCategoryMapper {}

impl CategoryMapper for MLBCategoryMapper {
    fn get_all_categories() -> Vec<String> {
        vec![ // TODO: this is not comprehensive
            String::from("P"),
            String::from("C"),
            String::from("1B"),
            String::from("2B"),
            String::from("3B"),
            String::from("SS"),
            String::from("OF"),
            String::from("CPT"),
        ]
    }
}

struct NFLCategoryMapper {}

impl CategoryMapper for NFLCategoryMapper {
    fn get_all_categories() -> Vec<String> {
        vec![ // TODO: this is not comprehensive
            String::from("QB"),
            String::from("RB"),
            String::from("WR"),
            String::from("TE"),
            String::from("FLEX"),
            String::from("DST"),
            String::from("CPT"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_each_nba() {
        let mapper: HashMap<String, u32> = get_nba_mapper();
        let mut pos = "PG";
        let mut cats: u32 = *mapper.get(pos).unwrap();
        assert!(cats == (NBAPositions::PointGuard as u32), "should contain item mapping to point guard");

        pos = "SG";
        cats = *mapper.get(pos).unwrap();
        assert!(cats == (NBAPositions::ShootingGuard as u32), "should contain item mapping to shooting guard");
    
        pos = "SF";
        cats = *mapper.get(pos).unwrap();
        assert!(cats == (NBAPositions::SmallForward as u32), "should contain item mapping to small forward");

        pos = "PF";
        cats = *mapper.get(pos).unwrap();
        assert!(cats == (NBAPositions::PowerForward as u32), "should contain item mapping to power forward");

        pos = "C";
        cats = *mapper.get(pos).unwrap();
        assert!(cats == (NBAPositions::Center as u32), "should contain item mapping to center");

        pos = "G";
        cats = *mapper.get(pos).unwrap();
        assert!(cats == (NBAPositions::Guard as u32), "should contain item mapping to guard");

        pos = "F";
        cats = *mapper.get(pos).unwrap();
        assert!(cats == (NBAPositions::Forward as u32), "should contain item mapping to foward");

        pos = "UTIL";
        cats = *mapper.get(pos).unwrap();
        assert!(cats == (NBAPositions::UTIL as u32), "should contain item mapping to UTIL");

        pos = "CPT";
        cats = *mapper.get(pos).unwrap();
        assert!(cats == (NBAPositions::Captain as u32), "should contain item mapping to captain");
    }

    #[test]
    fn multiple_unique_nba() {
        let positions: Vec<&str> = "PG/G/UTIL/CPT".split('/').collect();
        let mapper = get_nba_mapper();
        let mut cats: HashSet<u32> = HashSet::new();
        for pos in positions {
            cats.insert(*mapper.get(pos).unwrap());
        }
        assert!(cats.contains(&(NBAPositions::PointGuard as u32)));
        assert!(cats.contains(&(NBAPositions::Guard as u32)));
        assert!(cats.contains(&(NBAPositions::UTIL as u32)));
        assert!(cats.contains(&(NBAPositions::Captain as u32)));
    }

    #[test]
    fn multiple_repeat_nba() {
        let positions: Vec<&str> = "PG/PG/UTIL".split('/').collect();
        let mapper = get_nba_mapper();
        let mut cats: HashSet<u32> = HashSet::new();
        for pos in positions {
            cats.insert(*mapper.get(pos).unwrap());
        }
        assert!(cats.len() == 2);
        assert!(cats.contains(&(NBAPositions::PointGuard as u32)));
        assert!(cats.contains(&(NBAPositions::UTIL as u32)));
    }

    fn get_nba_mapper() -> HashMap<String, u32> {
        map_categories("nba").unwrap()
    }
}
