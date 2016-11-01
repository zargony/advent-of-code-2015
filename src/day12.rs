use std::str::{self, FromStr};

pub struct Numbers<'a> {
    iter: str::Chars<'a>,
}

impl<'a> Iterator for Numbers<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        let s: String = self.iter.by_ref().skip_while(|ch|
            !ch.is_digit(10) && *ch != '-'
        ).take_while(|ch|
            ch.is_digit(10) || *ch == '-'
        ).collect();
        if s.len() > 0 {
            Some(isize::from_str(&s).unwrap())
        } else {
            None
        }
    }
}

pub trait StrNumbersExt {
    fn numbers(&self) -> Numbers;
    fn numbers_sum(&self) -> isize;
}

impl StrNumbersExt for str {
    fn numbers(&self) -> Numbers {
        Numbers { iter: self.chars() }
    }

    fn numbers_sum(&self) -> isize {
        self.numbers().sum()
    }
}

fn main() {
    let input = include_str!("day12.txt");
    println!("Sum of all numbers: {}", input.numbers_sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summing_numbers() {
        assert_eq!(r#"[1,2,3]"#.numbers_sum(), 6);
        assert_eq!(r#"{"a":2,"b":4}"#.numbers_sum(), 6);
        assert_eq!(r#"[[[3]]]"#.numbers_sum(), 3);
        assert_eq!(r#"{"a":{"b":4},"c":-1}"#.numbers_sum(), 3);
        assert_eq!(r#"{"a":[-1,1]}"#.numbers_sum(), 0);
        assert_eq!(r#"[-1,{"a":1}]"#.numbers_sum(), 0);
        assert_eq!(r#"[]"#.numbers_sum(), 0);
        assert_eq!(r#"{}"#.numbers_sum(), 0);
    }
}
