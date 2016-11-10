#[macro_use]
extern crate nom;

use std::str::{self, FromStr};
use nom::{digit, space, eol};

named!(offset<isize>,
    map_res!(
        map_res!(
            alt!(
                recognize!(chain!(char!('-') ~ digit, || ())) |
                chain!(char!('+') ~ n: digit, || n)
            ),
            str::from_utf8
        ),
        FromStr::from_str
    )
);

#[derive(Debug, PartialEq, Eq)]
pub enum Register {
    A,
    B,
}

named!(register<Register>,
    alt!(
        value!(Register::A, tag!("a")) |
        value!(Register::B, tag!("b"))
    )
);

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

named!(instruction<Instruction>,
    alt!(
        chain!(tag!("hlf") ~ space ~ reg: register, || Instruction::Hlf(reg)) |
        chain!(tag!("tpl") ~ space ~ reg: register, || Instruction::Tpl(reg)) |
        chain!(tag!("inc") ~ space ~ reg: register, || Instruction::Inc(reg)) |
        chain!(tag!("jmp") ~ space ~ ofs: offset, || Instruction::Jmp(ofs)) |
        chain!(tag!("jie") ~ space ~ reg: register ~ tag!(",") ~ space ~ ofs: offset, || Instruction::Jie(reg, ofs)) |
        chain!(tag!("jio") ~ space ~ reg: register ~ tag!(",") ~ space ~ ofs: offset, || Instruction::Jio(reg, ofs))
    )
);

named!(pub program<Vec<Instruction> >,
    complete!(
        separated_list!(
            eol,
            instruction
        )
    )
);

#[derive(Debug, PartialEq, Eq)]
pub struct Vm {
    prog: Vec<Instruction>,
    a: usize,
    b: usize,
    ip: usize,
}

impl<'a> From<&'a str> for Vm {
    fn from(s: &str) -> Vm {
        Vm {
            prog: program(s.as_bytes()).unwrap().1,
            a: 0,
            b: 0,
            ip: 0,
        }
    }
}

impl Vm {
    fn jump(&mut self, offset: isize) {
        if offset < 0 {
            self.ip -= (-offset) as usize;
        } else {
            self.ip += offset as usize;
        }
    }

    fn done(&self) -> bool {
        self.ip >= self.prog.len()
    }

    fn step(&mut self) {
        match self.prog[self.ip] {
            Instruction::Hlf(Register::A) => { self.a /= 2; self.jump(1); },
            Instruction::Hlf(Register::B) => { self.b /= 2; self.jump(1); },
            Instruction::Tpl(Register::A) => { self.a *= 3; self.jump(1); },
            Instruction::Tpl(Register::B) => { self.b *= 3; self.jump(1); },
            Instruction::Inc(Register::A) => { self.a += 1; self.jump(1); },
            Instruction::Inc(Register::B) => { self.b += 1; self.jump(1); },
            Instruction::Jmp(ofs) => self.jump(ofs),
            Instruction::Jie(Register::A, ofs) => if self.a % 2 == 0 { self.jump(ofs) } else { self.jump(1) },
            Instruction::Jie(Register::B, ofs) => if self.b % 2 == 0 { self.jump(ofs) } else { self.jump(1) },
            Instruction::Jio(Register::A, ofs) => if self.a == 1 { self.jump(ofs) } else { self.jump(1) },
            Instruction::Jio(Register::B, ofs) => if self.b == 1 { self.jump(ofs) } else { self.jump(1) },
        }
    }

    fn run(&mut self) {
        while !self.done() {
            self.step();
        }
    }
}

fn main() {
    let mut vm = Vm::from(include_str!("day23.txt"));
    vm.run();
    println!("Value of register B after running program: {}", vm.b);
    let mut vm = Vm::from(include_str!("day23.txt"));
    vm.a = 1;
    vm.run();
    println!("Value of register B after running program if register A starts as 1: {}", vm.b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(program(b"inc a\njio a, +2\ntpl a\ninc a").unwrap(), (&b""[..], vec![
            Instruction::Inc(Register::A),
            Instruction::Jio(Register::A, 2),
            Instruction::Tpl(Register::A),
            Instruction::Inc(Register::A),
        ]));
    }

    #[test]
    fn parsing_complete() {
        let vm = Vm::from(include_str!("day23.txt"));
        assert_eq!(vm.prog.len(), 49);
    }

    #[test]
    fn stepping() {
        let mut vm = Vm::from("inc a\njio a, +2\ntpl a\ninc a");
        vm.step();
        assert_eq!(vm.a, 1);
        vm.step();
        assert_eq!(vm.ip, 3);
        vm.step();
        assert_eq!(vm.a, 2);
        assert!(vm.done());
    }

    #[test]
    fn running() {
        let mut vm = Vm::from("inc a\njio a, +2\ntpl a\ninc a");
        vm.run();
        assert_eq!(vm.a, 2);
        assert!(vm.done());
    }
}
