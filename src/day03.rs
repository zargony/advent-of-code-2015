use std::collections::HashSet;
use std::str;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    fn step(&mut self, step: char) {
        match step {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => panic!("invalid character"),
        }
    }
}

pub struct Visits {
    position: Position,
    visited: HashSet<Position>,
}

impl Visits {
    fn new() -> Visits {
        let mut visited = HashSet::new();
        visited.insert(Position::new());
        Visits { position: Position::new(), visited: visited }
    }

    fn step(&mut self, step: char) {
        self.position.step(step);
        self.visited.insert(self.position);
    }

    fn unique(&self) -> usize {
        self.visited.len()
    }
}

pub struct DuoVisits {
    position1: Position,
    position2: Position,
    visited: HashSet<Position>,
    second: bool,
}

impl DuoVisits {
    fn new() -> DuoVisits {
        let mut visited = HashSet::new();
        visited.insert(Position::new());
        DuoVisits { position1: Position::new(), position2: Position::new(), visited: visited, second: false }
    }

    fn step(&mut self, step: char) {
        if self.second {
            self.position1.step(step);
            self.visited.insert(self.position1);
        } else {
            self.position2.step(step);
            self.visited.insert(self.position2);
        }
        self.second = !self.second;
    }

    fn unique(&self) -> usize {
        self.visited.len()
    }
}

pub struct Directions<'a> {
    steps: &'a str,
}

impl<'a> Directions<'a> {
    fn new(steps: &str) -> Directions {
        Directions { steps: steps }
    }

    fn unique_visits(&self) -> usize {
        let mut visits = Visits::new();
        for step in self.steps.chars() {
            visits.step(step);
        }
        visits.unique()
    }

    fn unique_visits_with_robo(&self) -> usize {
        let mut visits = DuoVisits::new();
        for step in self.steps.chars() {
            visits.step(step);
        }
        visits.unique()
    }
}

fn main() {
    let directions = Directions::new(include_str!("day03.txt"));
    println!("Unique visits: {}", directions.unique_visits());
    println!("Unique visits with robo: {}", directions.unique_visits_with_robo());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_visits() {
        assert_eq!(Directions::new(">"         ).unique_visits(), 2);
        assert_eq!(Directions::new("^>v<"      ).unique_visits(), 4);
        assert_eq!(Directions::new("^v^v^v^v^v").unique_visits(), 2);
    }

    #[test]
    fn unique_visits_with_robo() {
        assert_eq!(Directions::new("^v"        ).unique_visits_with_robo(),  3);
        assert_eq!(Directions::new("^>v<"      ).unique_visits_with_robo(),  3);
        assert_eq!(Directions::new("^v^v^v^v^v").unique_visits_with_robo(), 11);
    }
}
