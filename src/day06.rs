#[macro_use]
extern crate nom;

use std::str::{self, FromStr};
use nom::{IResult, digit, space};

named!(number(&[u8]) -> usize,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

named!(command(&[u8]) -> Command,
    alt!(
        value!(Command::TurnOn, tag!("turn on")) |
        value!(Command::TurnOff, tag!("turn off")) |
        value!(Command::Toggle, tag!("toggle"))
    )
);

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    command: Command,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

named!(instruction(&[u8]) -> Instruction,
    chain!(
        cmd: command ~
        space ~
        x1: number ~
        tag!(",") ~
        y1: number ~
        space ~
        tag!("through") ~
        space ~
        x2: number ~
        tag!(",") ~
        y2: number,
        || Instruction { command: cmd, x1: x1, y1: y1, x2: x2, y2: y2 }
    )
);

pub trait Light {
    fn switch(&mut self, cmd: &Command);
    fn brightness(&self) -> usize;
}

impl Light for bool {
    fn switch(&mut self, cmd: &Command) {
        match cmd {
            &Command::TurnOn => *self = true,
            &Command::TurnOff => *self = false,
            &Command::Toggle => *self = !*self,
        }
    }

    fn brightness(&self) -> usize {
        match *self {
            false => 0,
            true => 1,
        }
    }
}

impl Light for u8 {
    fn switch(&mut self, cmd: &Command) {
        match cmd {
            &Command::TurnOn => *self += 1,
            &Command::TurnOff if *self > 0 => *self -= 1,
            &Command::TurnOff => (),
            &Command::Toggle => *self += 2,
        }
    }

    fn brightness(&self) -> usize {
        *self as usize
    }
}

pub struct Grid<T>([[T; 1000]; 1000]);

impl<T: Copy> Grid<T> {
    fn new(default: T) -> Grid<T> {
        Grid([[default; 1000]; 1000])
    }
}

impl<T: Light> Grid<T> {
    fn brightness(&self) -> usize {
        self.0.iter().map(|line|
            line.iter().map(|light| light.brightness()).sum::<usize>()
        ).sum()
    }

    fn change(&mut self, ins: &Instruction) {
        for y in ins.y1 .. ins.y2 + 1 {
            for x in ins.x1 .. ins.x2 + 1 {
                self.0[y][x].switch(&ins.command);
            }
        }
    }

    fn run_instructions(&mut self, text: &str) {
        for line in text.lines() {
            match instruction(line.as_bytes()) {
                IResult::Done(rest, ref ins) if rest == [] => self.change(ins),
                _ => panic!("invalid instruction in line '{}'", line),
            }
        }
    }
}

fn main() {
    let input = include_str!("day06.txt");
    let mut grid = Grid::new(false);
    grid.run_instructions(input);
    println!("Number of lit lights: {}", grid.brightness());
    let mut grid = Grid::new(0);
    grid.run_instructions(input);
    println!("Total brightness of all lights: {}", grid.brightness());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switching_lights() {
        let mut grid = Grid::new(false);
        assert_eq!(grid.brightness(), 0);
        grid.change(&Instruction { command: Command::TurnOn, x1: 0, y1: 0, x2: 999, y2: 999 });
        assert_eq!(grid.brightness(), 1_000_000);
        grid.change(&Instruction { command: Command::Toggle, x1: 0, y1: 0, x2: 999, y2: 0 });
        assert_eq!(grid.brightness(), 999_000);
        grid.change(&Instruction { command: Command::TurnOff, x1: 499, y1: 499, x2: 500, y2: 500 });
        assert_eq!(grid.brightness(), 998_996);
    }

    #[test]
    fn dimming_lights() {
        let mut grid = Grid::new(0u8);
        assert_eq!(grid.brightness(), 0);
        grid.change(&Instruction { command: Command::TurnOn, x1: 0, y1: 0, x2: 0, y2: 0 });
        assert_eq!(grid.brightness(), 1);
        grid.change(&Instruction { command: Command::Toggle, x1: 0, y1: 0, x2: 999, y2: 999 });
        assert_eq!(grid.brightness(), 2_000_001);
    }

    #[test]
    fn running_instructions() {
        let mut grid = Grid::new(false);
        grid.run_instructions("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500");
        assert_eq!(grid.brightness(), 998_996);
    }
}
