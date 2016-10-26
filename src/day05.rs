extern crate onig;

use onig::Regex;

pub trait Matcher {
    fn is_match(&self, s: &str) -> bool;
}

pub struct OldNicenessMatcher {
    re1: Regex,
    re2: Regex,
    re3: Regex,
}

impl OldNicenessMatcher {
    fn new() -> OldNicenessMatcher {
        OldNicenessMatcher {
            re1: Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap(),
            re2: Regex::new(r"([\w])\1").unwrap(),
            re3: Regex::new(r"(ab|cd|pq|xy)").unwrap(),
        }
    }
}

impl Matcher for OldNicenessMatcher {
    fn is_match(&self, s: &str) -> bool {
        self.re1.find(s).is_some() && self.re2.find(s).is_some() && !self.re3.find(s).is_some()
    }
}

pub struct NewNicenessMatcher {
    re1: Regex,
    re2: Regex,
}

impl NewNicenessMatcher {
    fn new() -> NewNicenessMatcher {
        NewNicenessMatcher {
            re1: Regex::new(r"(\w\w).*\1").unwrap(),
            re2: Regex::new(r"(\w)\w\1").unwrap(),
        }
    }
}

impl Matcher for NewNicenessMatcher {
    fn is_match(&self, s: &str) -> bool {
        self.re1.find(s).is_some() && self.re2.find(s).is_some()
    }
}

pub trait SantaStringExtension {
    fn is_nice<M: Matcher>(&self, m: &M) -> bool;
    fn is_naughty<M: Matcher>(&self, m: &M) -> bool { !self.is_nice(m) }
    fn nice_lines<M: Matcher>(&self, m: &M) -> usize;
}

impl<'a> SantaStringExtension for &'a str {
    fn is_nice<M: Matcher>(&self, m: &M) -> bool {
        m.is_match(self)
    }

    fn nice_lines<M: Matcher>(&self, m: &M) -> usize {
        self.lines().filter(|line| line.is_nice(m)).count()
    }
}

fn main() {
    let input = include_str!("day05.txt");
    let m = OldNicenessMatcher::new();
    println!("Nice strings (old rules): {}", input.lines().filter(|line| m.is_match(line)).count());
    let m = NewNicenessMatcher::new();
    println!("Nice strings (new rules): {}", input.lines().filter(|line| m.is_match(line)).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_niceness() {
        let m = OldNicenessMatcher::new();
        assert!("ugknbfddgicrmopn".is_nice(&m));
        assert!("aaa".is_nice(&m));
        assert!("jchzalrnumimnmhp".is_naughty(&m));
        assert!("haegwjzuvuyypxyu".is_naughty(&m));
        assert!("dvszwmarrgswjxmb".is_naughty(&m));
    }

    #[test]
    fn new_niceness() {
        let m = NewNicenessMatcher::new();
        assert!("qjhvhtzxzqqjkmpb".is_nice(&m));
        assert!("xxyxx".is_nice(&m));
        assert!("uurcxstgmygtbstg".is_naughty(&m));
        assert!("ieodomkazucvgmuy".is_naughty(&m));
    }
}
