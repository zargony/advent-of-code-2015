use std::str::FromStr;

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    pub fn new(sizespec: &str) -> Present {
        let sizes: Vec<u32> = sizespec.split('x').map(|s| u32::from_str(s).unwrap()).collect();
        assert!(sizes.len() == 3);
        Present { length: sizes[0], width: sizes[1], height: sizes[2] }
    }

    pub fn paper_size(&self) -> u32 {
        let sides = [self.length * self.width,
                     self.width  * self.height,
                     self.height * self.length];
        let extra = sides.iter().min().unwrap();
        sides.iter().fold(0, |sum, &side| sum + side) * 2 + extra
    }
}

pub struct Presents {
    presents: Vec<Present>,
}

impl Presents {
    pub fn new(sizespecs: &str) -> Presents {
        Presents { presents: sizespecs.lines().map(|line| Present::new(line)).collect() }
    }

    pub fn paper_size(&self) -> u32 {
        self.presents.iter().map(|p| p.paper_size()).fold(0, |sum, size| sum + size)
    }
}

fn main() {
    let presents = Presents::new(include_str!("day_02.txt"));
    println!("Total paper size: {}", presents.paper_size() );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paper_size() {
        assert_eq!(Present::new("2x3x4").paper_size(), 58);
        assert_eq!(Present::new("1x1x10").paper_size(), 43);
    }

    #[test]
    fn paper_size_sum() {
        assert_eq!(Presents::new("2x3x4\n1x1x10").paper_size(), 101);
    }
}
