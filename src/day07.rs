#[macro_use]
extern crate nom;

use std::cell::RefCell;
use std::collections::HashMap;
use std::str::{self, FromStr};
use nom::{IResult, alpha, digit, space};

named!(number(&[u8]) -> usize,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(wire(&[u8]) -> &str,
    map_res!(
        alpha,
        str::from_utf8
    )
);

#[derive(Debug, PartialEq, Eq)]
pub enum Value<'a> {
    Signal(u16),
    Wire(&'a str),
}

named!(pub value(&[u8]) -> Value,
    alt!(
        map!(number, |n| Value::Signal(n as u16))
        | map!(wire, |s| Value::Wire(s))
    )
);

#[derive(Debug, PartialEq, Eq)]
pub enum Expression<'a> {
    Not(Value<'a>),
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    LShift(Value<'a>, u8),
    RShift(Value<'a>, u8),
    Value(Value<'a>),
}

named!(pub expression(&[u8]) -> Expression,
    alt!(
        chain!(
            tag!("NOT") ~
            space ~
            v: value,
            || Expression::Not(v)
        ) | chain!(
            v1: value ~
            space ~
            tag!("AND") ~
            space ~
            v2: value,
            || Expression::And(v1, v2)
        ) | chain!(
            v1: value ~
            space ~
            tag!("OR") ~
            space ~
            v2: value,
            || Expression::Or(v1, v2)
        ) | chain!(
            v: value ~
            space ~
            tag!("LSHIFT") ~
            space ~
            n: number,
            || Expression::LShift(v, n as u8)
        ) | chain!(
            v: value ~
            space ~
            tag!("RSHIFT") ~
            space ~
            n: number,
            || Expression::RShift(v, n as u8)
        ) | chain!(
            v: value,
            || Expression::Value(v)
        )
    )
);

named!(pub assignment(&[u8]) -> (&str, Expression),
    complete!(
        chain!(
            e: expression ~
            space ~
            tag!("->") ~
            space ~
            w: wire,
            || (w, e)
        )
    )
);

pub struct Circuit<'a> {
    wires: HashMap<&'a str, Expression<'a>>,
    cache: RefCell<HashMap<&'a str, u16>>,
}

impl<'a> Circuit<'a> {
    fn new(definitions: &str) -> Circuit {
        let mut circ = Circuit { wires: HashMap::new(), cache: RefCell::new(HashMap::new()) };
        for line in definitions.lines() {
            match assignment(line.as_bytes()) {
                IResult::Done(_, (wire, expr)) => { circ.wires.insert(wire, expr); },
                _ => panic!("invalid assignment in line '{}'", line),
            }
        }
        circ
    }

    fn eval_value(&self, value: &Value<'a>) -> u16 {
        match *value {
            Value::Signal(s) => s,
            Value::Wire(w) => self.eval(w),
        }
    }

    fn eval_expression(&self, expr: &Expression<'a>) -> u16 {
        match *expr {
            Expression::Not(ref v) => !self.eval_value(v),
            Expression::And(ref v1, ref v2) => self.eval_value(v1) & self.eval_value(v2),
            Expression::Or(ref v1, ref v2) => self.eval_value(v1) | self.eval_value(v2),
            Expression::LShift(ref v, n) => self.eval_value(v) << n,
            Expression::RShift(ref v, n) => self.eval_value(v) >> n,
            Expression::Value(ref v) => self.eval_value(v),
        }
    }

    fn eval(&self, wire: &'a str) -> u16 {
        if let Some(res) = self.cache.borrow().get(wire) {
            return *res
        }
        match self.wires.get(wire) {
            Some(expr) => {
                let res = self.eval_expression(expr);
                self.cache.borrow_mut().insert(wire, res);
                res
            },
            None => panic!("unknown wire '{}'", wire),
        }
    }
}

fn main() {
    let circ = Circuit::new(include_str!("day07.txt"));
    println!("Ultimate signal to wire a: {}", circ.eval("a"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_value() {
        assert_eq!(value(b"123").unwrap(), (&b""[..], Value::Signal(123)));
        assert_eq!(value(b"foo").unwrap(), (&b""[..], Value::Wire("foo")));
    }

    #[test]
    fn parse_expression() {
        assert_eq!(expression(b"NOT foo").unwrap(), (&b""[..], Expression::Not(Value::Wire("foo"))));
        assert_eq!(expression(b"foo AND bar").unwrap(), (&b""[..], Expression::And(Value::Wire("foo"), Value::Wire("bar"))));
        assert_eq!(expression(b"foo OR bar").unwrap(), (&b""[..], Expression::Or(Value::Wire("foo"), Value::Wire("bar"))));
        assert_eq!(expression(b"foo LSHIFT 3").unwrap(), (&b""[..], Expression::LShift(Value::Wire("foo"), 3)));
        assert_eq!(expression(b"foo RSHIFT 4").unwrap(), (&b""[..], Expression::RShift(Value::Wire("foo"), 4)));
    }

    #[test]
    fn parse_assignment() {
        assert_eq!(assignment(b"123 -> x").unwrap(), (&b""[..], ("x", Expression::Value(Value::Signal(123)))));
        assert_eq!(assignment(b"x AND y -> z").unwrap(), (&b""[..], ("z", Expression::And(Value::Wire("x"), Value::Wire("y")))));
        assert_eq!(assignment(b"p LSHIFT 2 -> q").unwrap(), (&b""[..], ("q", Expression::LShift(Value::Wire("p"), 2))));
        assert_eq!(assignment(b"NOT e -> f").unwrap(), (&b""[..], ("f", Expression::Not(Value::Wire("e")))));
    }

    #[test]
    fn evaluate_value() {
        let circ = Circuit::new("123 -> x");
        assert_eq!(circ.eval_value(&Value::Signal(111)), 111);
        assert_eq!(circ.eval_value(&Value::Wire("x")), 123);
    }

    #[test]
    fn evaluate_expression() {
        let circ = Circuit::new("123 -> x");
        assert_eq!(circ.eval_expression(&Expression::Not(Value::Wire("x"))), 65412);
        assert_eq!(circ.eval_expression(&Expression::And(Value::Wire("x"), Value::Signal(14))), 10);
        assert_eq!(circ.eval_expression(&Expression::Or(Value::Wire("x"), Value::Signal(14))), 127);
        assert_eq!(circ.eval_expression(&Expression::LShift(Value::Wire("x"), 2)), 492);
        assert_eq!(circ.eval_expression(&Expression::RShift(Value::Wire("x"), 2)), 30);
    }

    #[test]
    fn evaluate() {
        let circ = Circuit::new("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i");
        assert_eq!(circ.eval("x"), 123);
        assert_eq!(circ.eval("y"), 456);
        assert_eq!(circ.eval("d"), 72);
        assert_eq!(circ.eval("e"), 507);
        assert_eq!(circ.eval("f"), 492);
        assert_eq!(circ.eval("g"), 114);
        assert_eq!(circ.eval("h"), 65412);
        assert_eq!(circ.eval("i"), 65079);
    }
}
