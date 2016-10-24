use std::collections::HashMap;
use std::str;

pub struct Visits<'a> {
    x: i32,
    y: i32,
    iter: &'a Iterator<Item=char>,
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
        Visits { x: 0, y: 0, iter: &self.steps.chars() }
    }

    pub fn visits_with_robo(&self) -> (Visits, Visits) {
        let odd_steps  = self.steps.chars().enumerate().filter_map(|(i, ch)| if i % 2 != 0 { Some(ch) } else { None } );
        let even_steps = self.steps.chars().enumerate().filter_map(|(i, ch)| if i % 2 == 0 { Some(ch) } else { None } );
        (Visits { x: 0, y: 0, iter: &odd_steps }, Visits { x: 0, y: 0, iter: &even_steps })
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

    pub fn unique_visits_with_robo(&self) -> usize {
        // let mut houses = HashMap::new();
        // houses.insert((0, 0), 2);
        // let (mut santa_visits, mut robo_visits) = self.visits().enumerate().partition(|&(i, _)| i % 2 == 0);
        // for coord in santa_visits {
        //     let counter = houses.entry(coord).or_insert(0);
        //     *counter += 1;
        // }
        // houses.len()
        0
    }
}

fn main() {
    let instructions = Instructions::new(include_str!("day03.txt"));
    println!("Unique visits: {}", instructions.unique_visits());
    println!("Unique visits with robo: {}", instructions.unique_visits_with_robo());
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

    #[test]
    fn unique_visits_with_robo() {
        assert_eq!(Instructions::new("^v"        ).unique_visits_with_robo(),  3);
        assert_eq!(Instructions::new("^>v<"      ).unique_visits_with_robo(),  3);
        assert_eq!(Instructions::new("^v^v^v^v^v").unique_visits_with_robo(), 11);
    }
}
