use std::str::FromStr;

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    pub fn new(sizespec: &str) -> Present {
        let mut sizes = sizespec.split('x').map(|s| u32::from_str(s).unwrap());
        Present { length: sizes.next().unwrap(), width: sizes.next().unwrap(), height: sizes.next().unwrap() }
    }

    pub fn sizes(&self) -> [u32; 3] {
        [self.length, self.width, self.height]
    }

    pub fn sides(&self) -> [u32; 3] {
        [self.length * self.width, self.width  * self.height, self.height * self.length]
    }

    pub fn smallest_side(&self) -> u32 {
        *self.sides().iter().min().unwrap()
    }

    pub fn paper_size(&self) -> u32 {
        self.sides().iter().fold(0, |sum, side| sum + side) * 2 + self.smallest_side()
    }

    pub fn ribbon_length(&self) -> u32 {
        let mut sizes = self.sizes();
        sizes.sort();
        2 * sizes[0] + 2 * sizes[1] + sizes.iter().fold(1, |prod, size| prod * size)
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

    pub fn ribbon_length(&self) -> u32 {
        self.presents.iter().map(|p| p.ribbon_length()).fold(0, |sum, size| sum + size)
    }
}

fn main() {
    let presents = Presents::new(include_str!("day02.txt"));
    println!("Total paper size: {}", presents.paper_size() );
    println!("Total ribbon length: {}", presents.ribbon_length() );
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

    #[test]
    fn ribbon_length() {
        assert_eq!(Present::new("2x3x4").ribbon_length(), 34);
        assert_eq!(Present::new("1x1x10").ribbon_length(), 14);
    }

    #[test]
    fn ribbon_length_sum() {
        assert_eq!(Presents::new("2x3x4\n1x1x10").ribbon_length(), 48);
    }
}
