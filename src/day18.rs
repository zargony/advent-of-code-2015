#[macro_use]
extern crate nom;

use std::fmt;
use nom::eol;

named!(line<Vec<bool> >,
    many1!(
        switch!(take!(1),
            b"#" => value!(true) |
            b"." => value!(false)
        )
    )
);

named!(lights<Vec<Vec<bool> > >,
    complete!(
        separated_list!(
            eol,
            line
        )
    )
);

#[derive(PartialEq, Eq)]
pub struct Grid(Vec<Vec<bool> >);

impl<'a> From<&'a str> for Grid {
    fn from(input: &str) -> Grid {
        Grid(lights(input.as_bytes()).unwrap().1)
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                match self.0[y][x] {
                    true => try!(f.write_str("#")),
                    false => try!(f.write_str(".")),
                }
            }
            try!(f.write_str("\n"));
        }
        Ok(())
    }
}

impl Grid {
    fn count(&self) -> usize {
        let mut count = 0;
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                match self.0[y][x] {
                    true => count += 1,
                    false => (),
                }
            }
        }
        count
    }

    fn lit_neighbors(&self, y: usize, x: usize) -> usize {
        fn lit(lights: &[Vec<bool>], y: isize, x: isize) -> bool {
            y >= 0 && y < lights.len() as isize &&
            x >= 0 && x < lights[y as usize].len() as isize &&
            lights[y as usize][x as usize]
        }
        [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter().map(|&(dy, dx)| {
            lit(&self.0, y as isize + dy, x as isize + dx)
        }).filter(|&l| l).count()
    }

    fn animate(&self) -> Grid {
        Grid((0..self.0.len()).map(|y| {
            (0..self.0[y].len()).map(|x| {
                match (self.0[y][x], self.lit_neighbors(y, x)) {
                    (true, 2...3) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false,
                }
            }).collect()
        }).collect())
    }

    fn animate_n(self, n: usize) -> Grid {
        (0..n).fold(self, |g, _| g.animate())
    }
}

fn main() {
    let grid = Grid::from(include_str!("day18.txt"));
    println!("Lit lights after 100 animation steps: {}", grid.animate_n(100).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_complete() {
        let grid = Grid::from(include_str!("day18.txt"));
        assert!(grid.count() > 0);
    }

    #[test]
    fn animating() {
        let grid = Grid::from(".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..");
        assert_eq!(grid.count(), 15);
        let grid = grid.animate();
        assert_eq!(grid.count(), 11);
        let grid = grid.animate();
        assert_eq!(grid.count(), 8);
        let grid = grid.animate();
        assert_eq!(grid.count(), 4);
        let grid = grid.animate();
        assert_eq!(grid.count(), 4);
    }

    #[test]
    fn multi_animating() {
        let grid = Grid::from(".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..");
        assert_eq!(grid.animate_n(4), Grid::from("......\n......\n..##..\n..##..\n......\n......"));
    }
}
