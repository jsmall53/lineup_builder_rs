use std::collections::{ HashSet };

pub trait CategoryMapper {
    fn map(string: &str) -> HashSet<usize>;
}

/// these are DFS positions for the NBA (at least on draft kings...)
enum NBAPositions {
    PointGuard = 0,
	ShootingGuard = 1,
	SmallForward = 2,
	PowerForward = 3,
	Center = 4,
	Guard = 5,
	Forward = 6,
	UTIL = 7, // how do I take this into account (wildcard), and futhermore how would I take a flex position into account RB/WR/TE
	Captain = 8,
}

pub struct NBACategoryMapper { }

impl NBACategoryMapper {
    fn get_pos_value(string: &str) -> usize {
        if string == "PG" {
            NBAPositions::PointGuard as usize
        } else if string == "SG" {
            NBAPositions::ShootingGuard as usize
        } else if string == "SF" {
            NBAPositions::SmallForward as usize
        } else if string == "PF" {
            NBAPositions::PowerForward as usize
        } else if string == "C" {
            NBAPositions::Center as usize
        } else if string == "G" {
            NBAPositions::Guard as usize
        } else if string == "F" {
            NBAPositions::Forward as usize
        } else if string == "UTIL" {
            NBAPositions::UTIL as usize
        } else if string == "CPT" {
            NBAPositions::Captain as usize
        } else {
            std::usize::MAX
        }
    }
}

impl CategoryMapper for NBACategoryMapper {
    fn map(string: &str) -> HashSet<usize> {
        let mut categories: HashSet<usize> = HashSet::new();
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
        let mut cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::PointGuard as usize)), "should contain item mapping to point guard");

        pos = "SG";
        cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::ShootingGuard as usize)), "should contain item mapping to shooting guard");
        
        pos = "SF";
        cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::SmallForward as usize)), "should contain item mapping to small forward");

        pos = "PF";
        cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::PowerForward as usize)), "should contain item mapping to power forward");

        pos = "C";
        cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::Center as usize)), "should contain item mapping to center");

        pos = "G";
        cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::Guard as usize)), "should contain item mapping to guard");

        pos = "F";
        cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::Forward as usize)), "should contain item mapping to foward");

        pos = "UTIL";
        cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::UTIL as usize)), "should contain item mapping to UTIL");

        pos = "CPT";
        cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 1, "should only have one entry");
        assert!(cats.contains(&(NBAPositions::Captain as usize)), "should contain item mapping to captain");
    }

    #[test]
    fn multiple_unique_nba() {
        let mut pos = "PG/G/UTIL/CPT";
        let cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 4);
        assert!(cats.contains(&(NBAPositions::PointGuard as usize)));
        assert!(cats.contains(&(NBAPositions::Guard as usize)));
        assert!(cats.contains(&(NBAPositions::UTIL as usize)));
        assert!(cats.contains(&(NBAPositions::Captain as usize)));
    }

    #[test]
    fn multiple_repeat_nba() {
        let mut pos = "PG/PG/UTIL";
        let cats = NBACategoryMapper::map(pos);
        assert!(cats.len() == 2);
        assert!(cats.contains(&(NBAPositions::PointGuard as usize)));
        assert!(cats.contains(&(NBAPositions::UTIL as usize)));
    }
}
