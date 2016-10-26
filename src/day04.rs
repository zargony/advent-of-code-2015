extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::fmt::Write;

static INPUT: &'static str = "iwrupvqb";

/// Check if a byte array has at least the given number of zero nibbles
pub fn has_zero_nibbles(bytes: &[u8], num_zeroes: usize) -> bool {
    bytes.iter().take(num_zeroes / 2).all(|b| *b == 0) &&
    (num_zeroes % 2 == 0 || *bytes.iter().nth(num_zeroes / 2).unwrap() < 0x10)
}

/// Finds the suffix that needs to be appended to the given prefix so that
/// the MD5 hash has at least the given number of leading zero nibbles
pub fn find_suffix(prefix: &str, num_zeroes: usize) -> u32 {
    let mut suffix = 1;
    let mut suffix_str = String::with_capacity(10);
    let mut sh_base = Md5::new();
    let mut digest = vec![0u8; sh_base.output_bytes()];
    sh_base.input_str(&prefix);
    loop {
        let mut sh = sh_base;
        suffix_str.clear();
        suffix_str.write_fmt(format_args!("{}", suffix)).unwrap();
        sh.input_str(&suffix_str);
        sh.result(&mut digest);
        if has_zero_nibbles(&digest, num_zeroes) {
            break;
        }
        suffix += 1;
    }
    suffix
}

fn main() {
    println!("Required suffix for 5 zeroes on hash for '{}': '{}'", INPUT, find_suffix(INPUT, 5));
    println!("Required suffix for 6 zeroes on hash for '{}': '{}'", INPUT, find_suffix(INPUT, 6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_check() {
        assert!(has_zero_nibbles(&[0x00, 0x01, 0x23, 0x45], 1));
        assert!(has_zero_nibbles(&[0x00, 0x01, 0x23, 0x45], 2));
        assert!(has_zero_nibbles(&[0x00, 0x01, 0x23, 0x45], 3));
        assert!(!has_zero_nibbles(&[0x00, 0x01, 0x23, 0x45], 4));
        assert!(has_zero_nibbles(&[0x00, 0x00, 0x23, 0x45], 3));
        assert!(has_zero_nibbles(&[0x00, 0x00, 0x23, 0x45], 4));
        assert!(!has_zero_nibbles(&[0x00, 0x00, 0x23, 0x45], 5));
        assert!(!has_zero_nibbles(&[0x00, 0x00, 0x23, 0x45], 6));
    }

    #[test]
    fn abcdef() {
        assert_eq!(find_suffix("abcdef", 5), 609043);
    }

    #[test]
    fn pqrstuv() {
        assert_eq!(find_suffix("pqrstuv", 5), 1048970);
    }

    // TIMINGS
    // with format! allocations on input and output of hashing:
    //   test tests::benchmark ... bench:  46,321,586 ns/iter (+/- 3,836,635)
    // with format! allocations on input, but no allocations on output:
    //   test tests::benchmark ... bench:  15,051,265 ns/iter (+/- 1,933,873)
    // with rust-crypto library instead:
    //   test tests::benchmark ... bench:  12,885,351 ns/iter (+/- 2,131,961)
    // without allocations on input as well:
    //   test tests::benchmark ... bench:   8,763,487 ns/iter (+/- 1,223,337)
    // without repeated re-hashing of prefix
    //   test tests::benchmark ... bench:   8,144,887 ns/iter (+/- 952,605)
    //
    // #[bench]
    // fn benchmark(b: &mut test::Bencher) {
    //     b.iter(|| find_suffix("abcdef", 4))
    // }
}
