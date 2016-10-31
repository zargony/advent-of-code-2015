use std::str;

pub fn ascii_increase(s: &mut [u8]) {
    if s.len() < 1 { panic!("unable to increase"); }
    let i = s.len() - 1;
    match s[i] {
        b'a'...b'y' => s[i] += 1,
        b'z' => {
            s[i] = b'a';
            ascii_increase(&mut s[0..i]);
        },
        _ => panic!("unable to increase"),
    }
}

pub fn password_has_increasing_straight(s: &[u8]) -> bool {
    let mut res = false;
    for i in 0..s.len() - 2 {
        if s[i] <= b'x' && s[i+1] == s[i] + 1 && s[i+2] == s[i] + 2 {
            res = true;
        }
    }
    res
}

pub fn password_has_no_confusing_letters(s: &[u8]) -> bool {
    !s.iter().any(|ch|
        *ch == b'i' || *ch == b'o' || *ch == b'l'
    )
}

pub fn password_has_two_pairs(s: &[u8]) -> bool {
    let mut pairs = 0;
    let mut i = 0;
    while i < s.len() - 1 {
        if s[i+1] == s[i] {
            pairs += 1;
            i += 1;
        }
        i += 1;
    }
    pairs >= 2
}

pub fn password_valid(s: &[u8]) -> bool {
    password_has_increasing_straight(s) && password_has_no_confusing_letters(s) && password_has_two_pairs(s)
}

pub fn next_password(s: &mut [u8]) {
    loop {
        ascii_increase(s);
        if password_valid(s) {
            break;
        }
    }
}

fn main() {
    let mut password = b"hepxcrrq".to_owned();
    next_password(&mut password);
    println!("Next password: {}", str::from_utf8(&password[..]).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increasing() {
        let mut s = b"xx".to_owned();
        ascii_increase(&mut s);
        assert_eq!(&s[..], b"xy");
        ascii_increase(&mut s);
        assert_eq!(&s[..], b"xz");
        ascii_increase(&mut s);
        assert_eq!(&s[..], b"ya");
        ascii_increase(&mut s);
        assert_eq!(&s[..], b"yb");
    }

    #[test]
    fn password_rules() {
        let s = b"hijklmmn";
        assert!(password_has_increasing_straight(s));
        assert!(!password_has_no_confusing_letters(s));
        let s = b"abbceffg";
        assert!(password_has_two_pairs(s));
        assert!(!password_has_increasing_straight(s));
        let s = b"abbcegjk";
        assert!(!password_has_two_pairs(s));
    }

    #[test]
    fn next_valid_password() {
        let mut s = b"abcdefgh".to_owned();
        next_password(&mut s);
        assert_eq!(&s[..], b"abcdffaa");
        let mut s = b"ghijklmn".to_owned();
        next_password(&mut s);
        assert_eq!(&s[..], b"ghjaabcc");
    }
}
