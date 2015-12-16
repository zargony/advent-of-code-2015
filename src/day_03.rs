use std::collections::HashMap;
use std::str;

pub struct Visits<'a> {
    x: i32,
    y: i32,
    iter: str::Chars<'a>,
}

impl<'a> Iterator for Visits<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        self.iter.next().map(|ch| {
            match ch {
                '^' => self.y += 1,
                'v' => self.y -= 1,
                '>' => self.x += 1,
                '<' => self.x -= 1,
                _ => (),
            }
            (self.x, self.y)
        })
    }
}

pub struct Instructions<'a> {
    steps: &'a str,
}

impl<'a> Instructions<'a> {
    pub fn new(steps: &str) -> Instructions {
        Instructions { steps: steps }
    }

    pub fn visits(&self) -> Visits {
        Visits { x: 0, y: 0, iter: self.steps.chars() }
    }

    pub fn unique_visits(&self) -> usize {
        let mut houses = HashMap::new();
        houses.insert((0, 0), 1);
        for coord in self.visits() {
            let counter = houses.entry(coord).or_insert(0);
            *counter += 1;
        }
        houses.len()
    }
}

fn main() {
    let instructions = Instructions::new(include_str!("day_03.txt"));
    println!("Unique visits: {}", instructions.unique_visits());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_visits() {
        assert_eq!(Instructions::new(">"         ).unique_visits(), 2);
        assert_eq!(Instructions::new("^>v<"      ).unique_visits(), 4);
        assert_eq!(Instructions::new("^v^v^v^v^v").unique_visits(), 2);
    }
}
