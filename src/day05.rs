#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

pub trait SantaStringExtension {
    fn is_nice(&self) -> bool;
    fn is_naughty(&self) -> bool { !self.is_nice() }
    fn nice_lines(&self) -> usize;
}

impl<'a> SantaStringExtension for &'a str {
    fn is_nice(&self) -> bool {
        lazy_static! {
            static ref RE1: Regex = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
            static ref RE2: Regex = Regex::new(r"(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz)").unwrap();
            static ref RE3: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
        }
        RE1.is_match(self) && RE2.is_match(self) && !RE3.is_match(self)
    }

    fn nice_lines(&self) -> usize {
        self.lines().filter(|line| line.is_nice()).count()
    }
}

fn main() {
    println!("Nice strings: {}", include_str!("day05.txt").nice_lines());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ugknbfddgicrmopn_is_nice() {
        assert!("ugknbfddgicrmopn".is_nice());
    }

    #[test]
    fn aaa_is_nice() {
        assert!("aaa".is_nice());
    }

    #[test]
    fn jchzalrnumimnmhp_is_naughty() {
        assert!("jchzalrnumimnmhp".is_naughty());
    }

    #[test]
    fn haegwjzuvuyypxyu_is_naughty() {
        assert!("haegwjzuvuyypxyu".is_naughty());
    }

    #[test]
    fn dvszwmarrgswjxmb_is_naughty() {
        assert!("dvszwmarrgswjxmb".is_naughty());
    }
}
