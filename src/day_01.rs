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
                _ => (),
            }
            self.floor
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

    pub fn floors(&self) -> Floors {
        Floors { floor: 0, iter: self.steps.chars() }
    }

    pub fn final_floor(&self) -> i32 {
        self.floors().last().unwrap()
    }

    pub fn basement_step(&self) -> Option<usize> {
        self.floors().position(|floor| floor < 0).map(|x| x+1)
    }
}

fn main() {
    let instructions = Instructions::new(include_str!("day_01.txt"));
    println!("Final floor: {}", instructions.final_floor());
    println!("Basement step: {}", instructions.basement_step().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_floor() {
        assert_eq!(Instructions::new("(())"   ).final_floor(),  0);
        assert_eq!(Instructions::new("()()"   ).final_floor(),  0);
        assert_eq!(Instructions::new("((("    ).final_floor(),  3);
        assert_eq!(Instructions::new("(()(()(").final_floor(),  3);
        assert_eq!(Instructions::new("))(((((").final_floor(),  3);
        assert_eq!(Instructions::new("())"    ).final_floor(), -1);
        assert_eq!(Instructions::new("))("    ).final_floor(), -1);
        assert_eq!(Instructions::new(")))"    ).final_floor(), -3);
        assert_eq!(Instructions::new(")())())").final_floor(), -3);
    }

    #[test]
    fn basement_step() {
        assert_eq!(Instructions::new(")"    ).basement_step(), Some(1));
        assert_eq!(Instructions::new("()())").basement_step(), Some(5));
    }
}
