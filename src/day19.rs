#[macro_use]
extern crate nom;

use std::{slice, str};
use std::collections::HashSet;
use std::iter::FromIterator;
use nom::{alphanumeric, space, eol};

named!(pub replacement<(&str, &str)>,
    chain!(
        from: map_res!(alphanumeric, str::from_utf8) ~
        space ~ tag!("=>") ~ space ~
        to: map_res!(alphanumeric, str::from_utf8),
        || (from, to)
    )
);

named!(medicine<(Vec<(&str, &str)>, &str)>,
    complete!(
        chain!(
            replacements: separated_list!(
                eol,
                replacement
            ) ~ eol ~ eol ~
            molecule: map_res!(alphanumeric, str::from_utf8),
            || (replacements, molecule)
        )
    )
);

pub struct Medicine<'a> {
    replacements: Vec<(&'a str, &'a str)>,
    molecule: &'a str,
}

impl<'a> From<&'a str> for Medicine<'a> {
    fn from(input: &str) -> Medicine {
        let (replacements, molecule) = medicine(input.as_bytes()).unwrap().1;
        Medicine { replacements: replacements, molecule: molecule }
    }
}

impl<'a> Medicine<'a> {
    fn molecules(&self) -> Molecules {
        Molecules::new(self)
    }

    fn count_distinct_molecules(&self) -> usize {
        HashSet::<String>::from_iter(self.molecules()).len()
    }
}

pub struct Molecules<'a> {
    medicine: &'a Medicine<'a>,
    replacements_iter: slice::Iter<'a, (&'a str, &'a str)>,
    replacement: Option<&'a (&'a str, &'a str)>,
    replace_iter: Option<str::MatchIndices<'a, &'a str>>,
}

impl<'a> Molecules<'a> {
    fn new(medicine: &'a Medicine) -> Molecules<'a> {
        Molecules {
            medicine: medicine,
            replacements_iter: medicine.replacements.iter(),
            replacement: None,
            replace_iter: None,
        }
    }
}

impl<'a> Iterator for Molecules<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut replace = match self.replace_iter {
            Some(ref mut it) => it.next(),
            None => None,
        };
        while replace.is_none() {
            self.replacement = self.replacements_iter.next();
            if self.replacement.is_none() {
                return None;
            }
            self.replace_iter = Some(self.medicine.molecule.match_indices(self.replacement.unwrap().0));
            replace = self.replace_iter.as_mut().unwrap().next();
        }
        Some(format!("{}{}{}",
            &self.medicine.molecule[0..replace.unwrap().0],
            self.replacement.unwrap().1,
            &self.medicine.molecule[replace.unwrap().0 + self.replacement.unwrap().0.len()..]
        ))
    }
}

fn main() {
    let medicine = Medicine::from(include_str!("day19.txt"));
    println!("Number of distinct molecules: {}", medicine.count_distinct_molecules());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "H => HO\nH => OH\nO => HH\n\nHOH";

    #[test]
    fn parsing() {
        assert_eq!(replacement(b"H => HO").unwrap(), (&b""[..], ("H", "HO")));
        let medicine = Medicine::from(INPUT);
        assert_eq!(medicine.replacements.len(), 3);
        assert_eq!(medicine.molecule, "HOH");
    }

    #[test]
    fn iterating_molecules() {
        let medicine = Medicine::from(INPUT);
        let mut molecules = medicine.molecules();
        assert_eq!(molecules.next(), Some("HOOH".to_owned()));
        assert_eq!(molecules.next(), Some("HOHO".to_owned()));
        assert_eq!(molecules.next(), Some("OHOH".to_owned()));
        assert_eq!(molecules.next(), Some("HOOH".to_owned()));
        assert_eq!(molecules.next(), Some("HHHH".to_owned()));
        assert_eq!(molecules.next(), None);
    }

    #[test]
    fn counting_distinct_molecules() {
        let medicine = Medicine::from(INPUT);
        assert_eq!(medicine.molecules().count(), 5);
        assert_eq!(medicine.count_distinct_molecules(), 4);
        let medicine = Medicine::from("H => HO\nH => OH\nO => HH\n\nHOHOHO");
        assert_eq!(medicine.molecules().count(), 9);
        assert_eq!(medicine.count_distinct_molecules(), 7);
    }
}
