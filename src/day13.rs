#[macro_use]
extern crate nom;

mod permute;

use std::str::{self, FromStr};
use nom::{alphanumeric, digit, space, eol};
use permute::PermutationExt;

named!(pub relation<(&str, &str, isize)>,
    chain!(
        person1: map_res!(alphanumeric, str::from_utf8) ~ space ~ tag!("would") ~ space ~
        sign: alt!(value!(1, tag!("gain")) | value!(-1, tag!("lose"))) ~ space ~
        happiness: map_res!(map_res!(digit, str::from_utf8), isize::from_str) ~ space ~
        tag!("happiness units by sitting next to") ~ space ~
        person2: map_res!(alphanumeric, str::from_utf8) ~ tag!("."),
        || (person1, person2, sign * happiness)
    )
);

named!(pub relations<Vec<(&str, &str, isize)> >,
    complete!(
        separated_list!(
            eol,
            relation
        )
    )
);

#[derive(Debug, PartialEq, Eq)]
pub struct GuestList<'a> {
    relations: Vec<(&'a str, &'a str, isize)>,
    people: Vec<&'a str>,
}

impl<'a> From<&'a str> for GuestList<'a> {
    fn from(input: &str) -> GuestList {
        let rels = relations(input.as_bytes()).unwrap().1;
        let mut ppl = Vec::new();
        for &(p1, p2, _) in &rels {
            if !ppl.contains(&p1) { ppl.push(&p1); }
            if !ppl.contains(&p2) { ppl.push(&p2); }
        }
        GuestList { relations: rels, people: ppl }
    }
}

impl<'a> GuestList<'a> {
    fn score_for(&self, person: &str, other: &str) -> Option<isize> {
        self.relations.iter().find(|&&(p1, p2, _)| {
            p1 == person && p2 == other
        }).map(|&(_, _, score)| score)
    }

    fn score(&self, seating_plan: &[&str]) -> isize {
        let len = seating_plan.len();
        let mut score = 0;
        for i in 1..len + 1 {
            score += self.score_for(seating_plan[i % len], seating_plan[i - 1]).unwrap_or(0);
            score += self.score_for(seating_plan[i % len], seating_plan[(i + 1) % len]).unwrap_or(0);
        }
        score
    }

    fn seating_plans(&self) -> permute::Permutations<&'a str> {
        self.people.permutations()
    }

    fn optimal_seating_plan(&self) -> (Vec<&str>, isize) {
        self.seating_plans().map(|seating_plan| {
            let score = self.score(&seating_plan);
            (seating_plan, score)
        }).max_by_key(|&(_, score)| {
            score
        }).unwrap()
    }
}

fn main() {
    let guest_list = GuestList::from(include_str!("day13.txt"));
    let (_, score) = guest_list.optimal_seating_plan();
    println!("Total change in happiness for optimal seating plan: {}", score);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "Alice would gain 54 happiness units by sitting next to Bob.\nAlice would lose 79 happiness units by sitting next to Carol.\nAlice would lose 2 happiness units by sitting next to David.\nBob would gain 83 happiness units by sitting next to Alice.\nBob would lose 7 happiness units by sitting next to Carol.\nBob would lose 63 happiness units by sitting next to David.\nCarol would lose 62 happiness units by sitting next to Alice.\nCarol would gain 60 happiness units by sitting next to Bob.\nCarol would gain 55 happiness units by sitting next to David.\nDavid would gain 46 happiness units by sitting next to Alice.\nDavid would lose 7 happiness units by sitting next to Bob.\nDavid would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn parsing() {
        assert_eq!(relations(INPUT.as_bytes()).unwrap(), (&b""[..], vec![
            ("Alice", "Bob",    54), ("Alice", "Carol", -79), ("Alice", "David",  -2),
            ("Bob",   "Alice",  83), ("Bob",   "Carol",  -7), ("Bob",   "David", -63),
            ("Carol", "Alice", -62), ("Carol", "Bob",    60), ("Carol", "David",  55),
            ("David", "Alice",  46), ("David", "Bob",    -7), ("David", "Carol",  41),
        ]));
    }

    #[test]
    fn parsing_complete() {
        let guest_list = GuestList::from(include_str!("day13.txt"));
        assert_eq!(guest_list.relations.len(), 56);
        assert_eq!(guest_list.people.len(), 8);
    }

    #[test]
    fn calculating_score() {
        let guest_list = GuestList::from(INPUT);
        assert_eq!(guest_list.score_for("Alice", "Bob"), Some(54));
        assert_eq!(guest_list.score_for("David", "Alice"), Some(46));
        assert_eq!(guest_list.score(&["Alice", "Bob", "Carol", "David"]), 330);
    }

    #[test]
    fn permuting_seating_plans() {
        let guest_list = GuestList::from(INPUT);
        let mut seating_plans = guest_list.seating_plans();
        assert_eq!(seating_plans.next(), Some(vec!["Alice", "Bob", "Carol", "David"]));
        // ...
    }

    #[test]
    fn finding_optimal_seating_plan() {
        let guest_list = GuestList::from(INPUT);
        assert_eq!(guest_list.optimal_seating_plan(), (vec!["Bob", "Carol", "David", "Alice"], 330));
    }
}
