pub fn code_recursive(row: usize, col: usize) -> usize {
    match (row, col) {
        (1, 1) => 20151125,
        (row, 1) => code_recursive(1, row - 1) * 252533 % 33554393,
        (row, col) => code_recursive(row + 1, col - 1) * 252533 % 33554393,
    }
}

pub fn code_number(row: usize, col: usize) -> usize {
    (1..row+col).fold(1, |no, row| no + row - 1) + col - 1
}

pub fn code_iterative(row: usize, col: usize) -> usize {
    let mut code = 20151125;
    for _ in 1..code_number(row, col) {
        code = code * 252533 % 33554393;
    }
    code
}

fn main() {
    let (row, col) = (2981, 3075);
    println!("Code for row {}, column {} is: {}", row, col, code_iterative(row, col));
}

#[cfg(test)]
mod tests {
    use super::*;

    const KNOWN_CODES: [[usize; 6]; 6] = [
        [20151125, 18749137, 17289845, 30943339, 10071777, 33511524],
        [31916031, 21629792, 16929656,  7726640, 15514188,  4041754],
        [16080970,  8057251,  1601130,  7981243, 11661866, 16474243],
        [24592653, 32451966, 21345942,  9380097, 10600672, 31527494],
        [   77061, 17552253, 28094349,  6899651,  9250759, 31663883],
        [33071741,  6796745, 25397450, 24659492,  1534922, 27995004],
    ];

    const CODE_NUMBERS: [[usize; 6]; 6] = [
        [ 1,  3,  6, 10, 15, 21],
        [ 2,  5,  9, 14, 20,  0],
        [ 4,  8, 13, 19,  0,  0],
        [ 7, 12, 18,  0,  0,  0],
        [11, 17,  0,  0,  0,  0],
        [16,  0,  0,  0,  0,  0],
    ];

    #[test]
    fn codes_recursive() {
        for row in 1..7 {
            for col in 1..7 {
                assert_eq!(code_recursive(row, col), KNOWN_CODES[row-1][col-1]);
            }
        }
    }

    #[test]
    fn code_numbers() {
        for row in 1..7 {
            for col in 1..8-row {
                assert_eq!(code_number(row, col), CODE_NUMBERS[row-1][col-1]);
            }
        }
    }

    #[test]
    fn codes_iterative() {
        for row in 1..7 {
            for col in 1..7 {
                assert_eq!(code_iterative(row, col), KNOWN_CODES[row-1][col-1]);
            }
        }
    }
}
