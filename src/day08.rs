extern crate onig;

use onig::Regex;

pub fn raw_and_unescaped_len(s: &str) -> (usize, usize) {
    if s.chars().nth(0) != Some('"') || s.chars().last() != Some('"') {
        panic!("invalid format (not quoted)");
    }
    let raw_len = s.len();
    let re = Regex::new(r#"\\(\\|"|x[0-9a-f]{2})"#).unwrap();
    let ss = &s[1..s.len()-1];
    let (esc_count, esc_size) = re.find_iter(ss).fold((0, 0), |(esc_count, esc_size), (start_pos, end_pos)| {
        (esc_count + 1, esc_size + (end_pos - start_pos))
    });
    (raw_len, raw_len - 2 - esc_size + esc_count)
}

fn main() {
    let extra_chars = include_str!("day08.txt").lines().fold(0, |extra_chars, line| {
        let (raw_len, unescaped_len) = raw_and_unescaped_len(line);
        extra_chars + (raw_len - unescaped_len)
    });
    println!("Number of extra characters: {}", extra_chars);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(raw_and_unescaped_len(r#""""#), (2, 0));
        assert_eq!(raw_and_unescaped_len(r#""abc""#), (5, 3));
        assert_eq!(raw_and_unescaped_len(r#""aaa\"aaa""#), (10, 7));
        assert_eq!(raw_and_unescaped_len(r#""\x27""#), (6, 1));
    }
}
