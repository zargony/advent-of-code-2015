#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::str::{self, FromStr};
use nom::{alphanumeric, digit, space, eol};

named!(pub route<(&str, &str, usize)>,
    chain!(
        from: map_res!(alphanumeric, str::from_utf8) ~
        space ~ tag!("to") ~ space ~
        to: map_res!(alphanumeric, str::from_utf8) ~
        space ~ tag!("=") ~ space ~
        dist: map_res!(map_res!(digit, str::from_utf8), FromStr::from_str),
        || (from, to, dist)
    )
);

named!(pub routes<Vec<(&str, &str, usize)> >,
    complete!(
        separated_list!(
            eol,
            route
        )
    )
);

#[derive(Debug, PartialEq, Eq)]
pub struct Routes<'a>(HashMap<&'a str, HashMap<&'a str, usize>>);

impl<'a> From<&'a str> for Routes<'a> {
    fn from(input: &str) -> Routes {
        let mut map: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
        for (from, to, dist) in routes(input.as_bytes()).unwrap().1 {
            map.entry(from).or_insert(HashMap::new()).insert(to, dist);
        }
        Routes(map)
    }
}

impl<'a> Routes<'a> {
    fn each_path<F: FnMut(&[&str], usize)>(&self, mut f: F) {
        fn each<F: FnMut(&[&str], usize)>(routes: &HashMap<&str, HashMap<&str, usize>>, path: &[&str], dist: usize, f: &mut F) {
            match routes.get(path.last().unwrap()) {
                Some(tos) => for (to, d) in tos {
                    if !path.contains(to) {
                        let mut new_path = path.to_owned();
                        new_path.push(to);
                        each(routes, &new_path, dist+d, f);
                    }
                },
                None => f(path, dist),
            }
        }
        for from in self.0.keys() {
            each(&self.0, &[from], 0, &mut f);
        }
    }

    fn max_path_len(&self) -> usize {
        let mut maxlen = 0;
        self.each_path(|path, _|
            if path.len() > maxlen { maxlen = path.len() }
        );
        maxlen
    }

    fn shortest_full_path_dist(&self) -> usize {
        let maxlen = self.max_path_len();
        let mut mindist = usize::max_value();
        self.each_path(|path, dist| {
            if path.len() == maxlen && dist < mindist { mindist = dist }
        });
        mindist
    }
}

fn main() {
    let routes = Routes::from(include_str!("day09.txt"));
    // XXX
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(routes(b"London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141").unwrap(),
            (&b""[..], vec![("London", "Dublin", 464), ("London", "Belfast", 518), ("Dublin", "Belfast", 141)]));
    }

    #[test]
    fn parsing_complete() {
        let routes = Routes::from(include_str!("day09.txt"));
        assert_eq!(routes.0.len(), 7);
    }

    #[test]
    fn permuting() {
        let routes = Routes::from("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141");
        let mut paths: Vec<(Vec<String>, usize)> = Vec::new();
        routes.each_path(|path, dist| {
            paths.push((path.iter().map(|s| String::from(*s)).collect(), dist));
        });
        // assert_eq!(paths.len(), 6);
        // assert!(paths.contains(&(vec!["Dublin".to_owned(), "London".to_owned(), "Belfast".to_owned()], 982)));
        // assert!(paths.contains(&(vec!["London".to_owned(), "Dublin".to_owned(), "Belfast".to_owned()], 605)));
        // assert!(paths.contains(&(vec!["London".to_owned(), "Belfast".to_owned(), "Dublin".to_owned()], 659)));
        // assert!(paths.contains(&(vec!["Dublin".to_owned(), "Belfast".to_owned(), "London".to_owned()], 659)));
        // assert!(paths.contains(&(vec!["Belfast".to_owned(), "Dublin".to_owned(), "London".to_owned()], 605)));
        // assert!(paths.contains(&(vec!["Belfast".to_owned(), "London".to_owned(), "Dublin".to_owned()], 982)));
    }

    #[test]
    fn longest_path() {
        let routes = Routes::from("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141");
        assert_eq!(routes.max_path_len(), 3);
    }

    #[test]
    fn shortest_full_distance() {
        let routes = Routes::from("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141");
        assert_eq!(routes.shortest_full_path_dist(), 605);
    }
}
