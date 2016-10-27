#[macro_use]
extern crate nom;

use std::str::{self, FromStr};
use nom::{IResult, digit, space};

named!(number <usize>,
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

named!(command <Command>,
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

named!(instruction <Instruction>,
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

pub struct Grid([[bool; 1000]; 1000]);

impl Grid {
    fn new() -> Grid {
        Grid([[false; 1000]; 1000])
    }

    fn count(&self) -> usize {
        self.0.iter().map(|line|
            line.iter().filter(|light| **light).count()
        ).sum()
    }

    fn change(&mut self, ins: &Instruction) {
        for y in ins.y1 .. ins.y2 + 1 {
            for x in ins.x1 .. ins.x2 + 1 {
                match ins.command {
                    Command::TurnOn => self.0[y][x] = true,
                    Command::TurnOff => self.0[y][x] = false,
                    Command::Toggle => self.0[y][x] = !self.0[y][x],
                }
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
    let mut grid = Grid::new();
    grid.run_instructions(include_str!("day06.txt"));
    println!("Number of lit lights: {}", grid.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switching_lights() {
        let mut grid = Grid::new();
        assert_eq!(grid.count(), 0);
        grid.change(&Instruction { command: Command::TurnOn, x1: 0, y1: 0, x2: 999, y2: 999 });
        assert_eq!(grid.count(), 1_000_000);
        grid.change(&Instruction { command: Command::Toggle, x1: 0, y1: 0, x2: 999, y2: 0 });
        assert_eq!(grid.count(), 999_000);
        grid.change(&Instruction { command: Command::TurnOff, x1: 499, y1: 499, x2: 500, y2: 500 });
        assert_eq!(grid.count(), 998_996);
    }

    #[test]
    fn running_instructions() {
        let mut grid = Grid::new();
        grid.run_instructions("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500");
        assert_eq!(grid.count(), 998_996);
    }
}
