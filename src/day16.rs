#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::{self, FromStr};
use nom::{alphanumeric, digit, eol};

named!(name<&str>,
    map_res!(
        alphanumeric,
        str::from_utf8
    )
);

named!(number<usize>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

#[derive(Debug, PartialEq, Eq)]
pub struct Aunt<'a> {
    name: &'a str,
    compounds: HashMap<&'a str, usize>,
}

named!(pub aunt<Aunt>,
    chain!(
        aunt: map_res!(take_until!(":"), str::from_utf8) ~
        tag!(": ") ~
        compounds: separated_list!(
            tag!(", "),
            separated_pair!(
                name,
                tag!(": "),
                number
            )
        ),
        || Aunt { name: aunt, compounds: HashMap::from_iter(compounds) }
    )
);

named!(pub aunts<Vec<Aunt> >,
    complete!(
        separated_list!(
            eol,
            aunt
        )
    )
);

fn match_criteria(compounds: &HashMap<&str, usize>, criteria: &[(&str, usize)]) -> bool {
    for &(comp, count) in criteria {
        if compounds.contains_key(comp) && compounds.get(comp) != Some(&count) {
            return false;
        }
    }
    true
}

fn main() {
    let aunts = aunts(include_str!("day16.txt").as_bytes()).unwrap().1;
    for aunt in aunts {
        if match_criteria(&aunt.compounds, &[
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ]) {
            println!("Found matching aunt: {}", aunt.name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn parsing() {
        let mut compounds: HashMap<&str, usize> = HashMap::new();
        compounds.insert("goldfish", 6);
        compounds.insert("trees", 9);
        compounds.insert("akitas", 0);
        assert_eq!(aunt(b"Sue 1: goldfish: 6, trees: 9, akitas: 0").unwrap(),
            (&b""[..], Aunt { name: "Sue 1", compounds: compounds }));
    }

    #[test]
    fn parsing_complete() {
        assert_eq!(aunts(include_str!("day16.txt").as_bytes()).unwrap().1.len(), 500);
    }
}
