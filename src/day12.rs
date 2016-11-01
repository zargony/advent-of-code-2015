#[macro_use]
extern crate nom;

use std::str::{self, FromStr};
use nom::{alphanumeric, digit};

#[derive(Debug, PartialEq, Eq)]
pub enum JsonObject<'a> {
    Number(isize),
    String(&'a str),
    Array(Vec<JsonObject<'a>>),
    Hash(Vec<(&'a str, JsonObject<'a>)>),
}

named!(pub json_number(&[u8]) -> isize,
    map_res!(
        map_res!(
            recognize!(chain!(
                opt!(char!('-')) ~
                digit,
                || ()
            )),
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(pub json_string(&[u8]) -> &str,
    map_res!(
        delimited!(
            char!('"'),
            alphanumeric,
            char!('"')
        ),
        str::from_utf8
    )
);

named!(pub json_array(&[u8]) -> Vec<JsonObject>,
    delimited!(
        char!('['),
        separated_list!(
            char!(','),
            json
        ),
        char!(']')
    )
);

named!(pub json_hash(&[u8]) -> Vec<(&str, JsonObject)>,
    delimited!(
        char!('{'),
        separated_list!(
            char!(','),
            separated_pair!(
                json_string,
                char!(':'),
                json
            )
        ),
        char!('}')
    )
);

named!(pub json(&[u8]) -> JsonObject,
    alt!(
        map!(json_number, |n| JsonObject::Number(n)) |
        map!(json_string, |s| JsonObject::String(s)) |
        map!(json_array, |a| JsonObject::Array(a)) |
        map!(json_hash, |h| JsonObject::Hash(h))
    )
);

named!(pub complete_json(&[u8]) -> JsonObject,
    complete!(json)
);

impl<'a> JsonObject<'a> {
    fn parse(input: &str) -> JsonObject {
        complete_json(input.as_bytes()).unwrap().1
    }

    fn sum_numbers(&self) -> isize {
        match *self {
            JsonObject::Number(n) => n,
            JsonObject::Array(ref a) => a.iter().map(|e| e.sum_numbers()).sum(),
            JsonObject::Hash(ref h) => h.iter().map(|&(_, ref e)| e.sum_numbers()).sum(),
            _ => 0,
        }
    }
}

pub trait StrJsonExt {
    fn to_json(&self) -> JsonObject;
}

impl<T: AsRef<str>> StrJsonExt for T {
    fn to_json(&self) -> JsonObject {
        JsonObject::parse(self.as_ref())
    }
}

fn main() {
    let input = include_str!("day12.txt");
    println!("Sum of all numbers: {}", input.to_json().sum_numbers());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(json_number(b"123").unwrap(), (&b""[..], 123));
        assert_eq!(json_number(b"-123").unwrap(), (&b""[..], -123));
        assert_eq!(json_string(br#""foo""#).unwrap(), (&b""[..], "foo"));
        assert_eq!(json_array(br#"[123,"foo"]"#).unwrap(), (&b""[..], vec![JsonObject::Number(123), JsonObject::String("foo")]));
        assert_eq!(json_hash(br#"{"a":123,"b":"foo"}"#).unwrap(), (&b""[..], vec![("a", JsonObject::Number(123)), ("b", JsonObject::String("foo"))]));
    }

    #[test]
    fn summing_numbers() {
        assert_eq!(r#"[1,2,3]"#.to_json().sum_numbers(), 6);
        assert_eq!(r#"{"a":2,"b":4}"#.to_json().sum_numbers(), 6);
        assert_eq!(r#"[[[3]]]"#.to_json().sum_numbers(), 3);
        assert_eq!(r#"{"a":{"b":4},"c":-1}"#.to_json().sum_numbers(), 3);
        assert_eq!(r#"{"a":[-1,1]}"#.to_json().sum_numbers(), 0);
        assert_eq!(r#"[-1,{"a":1}]"#.to_json().sum_numbers(), 0);
        assert_eq!(r#"[]"#.to_json().sum_numbers(), 0);
        assert_eq!(r#"{}"#.to_json().sum_numbers(), 0);
    }
}
