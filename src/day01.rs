use std::str;

pub struct Floors<'a> {
    floor: i32,
    iter: str::Chars<'a>,
}

impl<'a> Iterator for Floors<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.iter.next().map(|ch| {
            match ch {
                '(' => self.floor += 1,
                ')' => self.floor -= 1,
                _ => panic!("invalid character"),
            }
            self.floor
        })
    }
}

pub struct Directions<'a> {
    steps: &'a str,
}

impl<'a> Directions<'a> {
    fn new(steps: &str) -> Directions {
        Directions { steps: steps }
    }

    fn floors(&self) -> Floors {
        Floors { floor: 0, iter: self.steps.chars() }
    }

    fn final_floor(&self) -> Option<i32> {
        self.floors().last()
    }

    fn basement_step(&self) -> Option<usize> {
        self.floors().position(|floor| floor < 0).map(|x| x + 1)
    }
}

fn main() {
    let directions = Directions::new(include_str!("day01.txt"));
    println!("Final floor: {}", directions.final_floor().unwrap());
    println!("Basement step: {}", directions.basement_step().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_floor() {
        assert_eq!(Directions::new("(())"   ).final_floor().unwrap(),  0);
        assert_eq!(Directions::new("()()"   ).final_floor().unwrap(),  0);
        assert_eq!(Directions::new("((("    ).final_floor().unwrap(),  3);
        assert_eq!(Directions::new("(()(()(").final_floor().unwrap(),  3);
        assert_eq!(Directions::new("))(((((").final_floor().unwrap(),  3);
        assert_eq!(Directions::new("())"    ).final_floor().unwrap(), -1);
        assert_eq!(Directions::new("))("    ).final_floor().unwrap(), -1);
        assert_eq!(Directions::new(")))"    ).final_floor().unwrap(), -3);
        assert_eq!(Directions::new(")())())").final_floor().unwrap(), -3);
    }

    #[test]
    fn basement_step() {
        assert_eq!(Directions::new(")"    ).basement_step().unwrap(), 1);
        assert_eq!(Directions::new("()())").basement_step().unwrap(), 5);
    }
}
