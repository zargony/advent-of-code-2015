use std::char;
use std::iter;
use std::str;

pub struct Sequence<T: Iterator> {
    iter: iter::Peekable<T>,
    ch: Option<char>,
}

impl<T: Iterator<Item=char>> Sequence<T> {
    fn new(it: T) -> Sequence<T> {
        Sequence { iter: it.peekable(), ch: None }
    }
}

impl<T: Iterator<Item=char>> Iterator for Sequence<T> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.ch {
            Some(ch) => {
                self.ch = None;
                Some(ch)
            },
            None => match self.iter.next() {
                Some(ch) => {
                    self.ch = Some(ch);
                    let mut count = 1;
                    while self.iter.peek() == Some(&ch) {
                        self.iter.next();
                        count += 1;
                    }
                    Some(char::from_digit(count, 10).unwrap())
                },
                None => None,
            }
        }
    }
}

pub trait IteratorSequenceExt<T: Iterator> {
    fn sequence(self) -> Sequence<T>;
}

impl<T: Iterator<Item=char>> IteratorSequenceExt<T> for T {
    fn sequence(self) -> Sequence<T> {
        Sequence::new(self)
    }
}

fn main() {
    let input = "1321131112";
    let result = input.chars()
                    .sequence().sequence().sequence().sequence().sequence()
                    .sequence().sequence().sequence().sequence().sequence()
                    .sequence().sequence().sequence().sequence().sequence()
                    .sequence().sequence().sequence().sequence().sequence()
                    .sequence().sequence().sequence().sequence().sequence()
                    .sequence().sequence().sequence().sequence().sequence()
                    .sequence().sequence().sequence().sequence().sequence()
                    .sequence().sequence().sequence().sequence().sequence()
                    .collect::<String>();
    println!("Length of 40 times sequenced '1321131112': {}", result.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequencing() {
        let mut it = "211".chars().sequence();
        assert_eq!(it.next(), Some('1'));
        assert_eq!(it.next(), Some('2'));
        assert_eq!(it.next(), Some('2'));
        assert_eq!(it.next(), Some('1'));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn repeated_sequencing() {
        let s = "1".chars().sequence().collect::<String>();
        assert_eq!(s, "11");
        let s = s.chars().sequence().collect::<String>();
        assert_eq!(s, "21");
        let s = s.chars().sequence().collect::<String>();
        assert_eq!(s, "1211");
        let s = s.chars().sequence().collect::<String>();
        assert_eq!(s, "111221");
        let s = s.chars().sequence().collect::<String>();
        assert_eq!(s, "312211");
    }
}
