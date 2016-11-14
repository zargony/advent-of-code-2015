#[macro_use]
extern crate nom;

mod permute;

use std::str::{self, FromStr};
use nom::{alphanumeric, digit, space, eol};
use permute::PermutationExt;

named!(pub segment<(&str, &str, usize)>,
    chain!(
        from: map_res!(alphanumeric, str::from_utf8) ~
        space ~ tag!("to") ~ space ~
        to: map_res!(alphanumeric, str::from_utf8) ~
        space ~ tag!("=") ~ space ~
        dist: map_res!(map_res!(digit, str::from_utf8), FromStr::from_str),
        || (from, to, dist)
    )
);

named!(pub segments<Vec<(&str, &str, usize)> >,
    complete!(
        separated_list!(
            eol,
            segment
        )
    )
);

#[derive(Debug, PartialEq, Eq)]
pub struct Router<'a> {
    segments: Vec<(&'a str, &'a str, usize)>,
    locations: Vec<&'a str>,
}

impl<'a> From<&'a str> for Router<'a> {
    fn from(input: &str) -> Router {
        let segs = segments(input.as_bytes()).unwrap().1;
        let mut locs = Vec::new();
        for &(from, to, _) in &segs {
            if !locs.contains(&from) { locs.push(&from); }
            if !locs.contains(&to) { locs.push(&to); }
        }
        Router { segments: segs, locations: locs }
    }
}

impl<'a> Router<'a> {
    fn distance_between(&self, from: &str, to: &str) -> Option<usize> {
        self.segments.iter().find(|&&(loc1, loc2, _)| {
            (loc1 == from && loc2 == to) || (loc2 == from && loc1 == to)
        }).map(|&(_, _, dist)| dist)
    }

    fn distance(&self, route: &[&str]) -> Option<usize> {
        let mut dist = 0;
        for i in 0..route.len() - 1 {
            match self.distance_between(route[i], route[i+1]) {
                Some(d) => dist += d,
                None => return None,
            }
        }
        Some(dist)
    }

    fn routes(&self) -> permute::Permutations<&'a str> {
        self.locations.permutations()
    }

    fn shortest_route(&self) -> (Vec<&str>, usize) {
        self.routes().map(|route| {
            let dist = self.distance(&route).unwrap();
            (route, dist)
        }).min_by_key(|&(_, dist)| {
            dist
        }).unwrap()
    }

    fn longest_route(&self) -> (Vec<&str>, usize) {
        self.routes().map(|route| {
            let dist = self.distance(&route).unwrap();
            (route, dist)
        }).max_by_key(|&(_, dist)| {
            dist
        }).unwrap()
    }
}

fn main() {
    let router = Router::from(include_str!("day09.txt"));
    let (_, dist) = router.shortest_route();
    println!("Distance of shortest route: {}", dist);
    let (_, dist) = router.longest_route();
    println!("Distance of longest route: {}", dist);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

    #[test]
    fn parsing() {
        assert_eq!(segments(INPUT.as_bytes()).unwrap(),
            (&b""[..], vec![("London", "Dublin", 464), ("London", "Belfast", 518), ("Dublin", "Belfast", 141)]));
    }

    #[test]
    fn parsing_complete() {
        let router = Router::from(include_str!("day09.txt"));
        assert_eq!(router.segments.len(), 28);
        assert_eq!(router.locations.len(), 8);
    }

    #[test]
    fn permuting_routes() {
        let router = Router::from(INPUT);
        let mut routes = router.routes();
        assert_eq!(routes.next(), Some(vec!["London", "Dublin", "Belfast"]));
        assert_eq!(routes.next(), Some(vec!["Dublin", "London", "Belfast"]));
        assert_eq!(routes.next(), Some(vec!["Belfast", "London", "Dublin"]));
        assert_eq!(routes.next(), Some(vec!["London", "Belfast", "Dublin"]));
        assert_eq!(routes.next(), Some(vec!["Dublin", "Belfast", "London"]));
        assert_eq!(routes.next(), Some(vec!["Belfast", "Dublin", "London"]));
        assert_eq!(routes.next(), None);
    }

    #[test]
    fn calculating_distance() {
        let router = Router::from(INPUT);
        assert_eq!(router.distance(&["London", "Dublin", "Belfast"]), Some(605));
        assert_eq!(router.distance(&["Dublin", "London", "Belfast"]), Some(982));
        assert_eq!(router.distance(&["Belfast", "London", "Dublin"]), Some(982));
        assert_eq!(router.distance(&["London", "Belfast", "Dublin"]), Some(659));
        assert_eq!(router.distance(&["Dublin", "Belfast", "London"]), Some(659));
        assert_eq!(router.distance(&["Belfast", "Dublin", "London"]), Some(605));
    }

    #[test]
    fn finding_shortest_route() {
        let router = Router::from(INPUT);
        assert_eq!(router.shortest_route(), (vec!["London", "Dublin", "Belfast"], 605));
    }

    #[test]
    fn finding_longest_route() {
        let router = Router::from(INPUT);
        assert_eq!(router.longest_route(), (vec!["Belfast", "London", "Dublin"], 982));
    }
}
