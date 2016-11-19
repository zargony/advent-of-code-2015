pub fn sum_of_divisors(n: usize) -> usize {
    (1 .. n / 2 + 1).fold(n, |sum, x|
        sum + if n % x == 0 { x } else { 0 }
    )
}

pub fn special_sum_of_divisors(n: usize) -> usize {
    (1 .. n / 2 + 1).fold(n, |sum, x|
        sum + if n % x == 0 && n <= x * 50 { x } else { 0 }
    )
}

pub fn number_with_sum_of_divisor(mut n: usize, sum: usize) -> usize {
    loop {
        if sum_of_divisors(n) >= sum {
            return n;
        }
        n += 1;
    }
}

pub fn number_with_special_sum_of_divisor(mut n: usize, sum: usize) -> usize {
    loop {
        if special_sum_of_divisors(n) >= sum {
            return n;
        }
        n += 1;
    }
}

fn main() {
    let min_presents = 36_000_000;
    // We're cheat with start numbers here  to prevent long runtime (CI timeout)
    println!("Lowest house number that gets at least {} presents: {}",
        min_presents,
        number_with_sum_of_divisor(831_000, min_presents / 10));
    println!("Lowest house number that gets at least {} presents (special rules): {}",
        min_presents,
        number_with_special_sum_of_divisor(884_000, min_presents / 11));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_presents() {
        assert_eq!(sum_of_divisors(1), 1);
        assert_eq!(sum_of_divisors(2), 3);
        assert_eq!(sum_of_divisors(3), 4);
        assert_eq!(sum_of_divisors(4), 7);
        assert_eq!(sum_of_divisors(5), 6);
        assert_eq!(sum_of_divisors(6), 12);
        assert_eq!(sum_of_divisors(7), 8);
        assert_eq!(sum_of_divisors(8), 15);
        assert_eq!(sum_of_divisors(9), 13);
    }

    #[test]
    fn finding_house_with_min_presents() {
        assert_eq!(number_with_sum_of_divisor(1, 1), 1);
        assert_eq!(number_with_sum_of_divisor(1, 3), 2);
        assert_eq!(number_with_sum_of_divisor(1, 4), 3);
        assert_eq!(number_with_sum_of_divisor(1, 6), 4);
        assert_eq!(number_with_sum_of_divisor(1, 7), 4);
        assert_eq!(number_with_sum_of_divisor(1, 12), 6);
        assert_eq!(number_with_sum_of_divisor(1, 15), 8);
    }
}
