use std::slice;
use std::collections::HashMap;
use std::str::FromStr;

pub fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|line| {
        usize::from_str(line).unwrap()
    }).collect()
}

pub struct Permutations<'a> {
    items: Vec<usize>,
    iters: Vec<slice::Iter<'a, usize>>,
}

impl<'a> Permutations<'a> {
    fn new(items: &[usize]) -> Permutations {
        Permutations { items: vec![], iters: vec![items.iter()] }
    }
}

impl<'a> Iterator for Permutations<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        match self.iters.pop() {
            Some(mut iter) => {
                self.items.pop();
                match iter.next() {
                    Some(item) => {
                        self.items.push(*item);
                        self.iters.push(iter.clone());
                        while let Some(item) = iter.next() {
                            self.items.push(*item);
                            self.iters.push(iter.clone());
                        }
                    },
                    None => (),
                }
                match self.items.len() {
                    0 => None,
                    _ => Some(self.items.clone()),
                }
            },
            None => None,
        }
    }
}

pub struct PermutationsWithSum<'a> {
    iter: Permutations<'a>,
    sum: usize,
}

impl<'a> PermutationsWithSum<'a> {
    fn new(items: &[usize], sum: usize) -> PermutationsWithSum {
        PermutationsWithSum { iter: Permutations::new(items), sum: sum }
    }
}

impl<'a> Iterator for PermutationsWithSum<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        while let Some(items) = self.iter.next() {
            if items.iter().sum::<usize>() == self.sum { return Some(items) }
        }
        None
    }
}

pub fn count_smallest<I: Iterator<Item=Vec<usize>>>(iter: I) -> (usize, usize) {
    let mut sizes_count: HashMap<usize, usize> = HashMap::new();
    for items in iter {
        let size = items.len();
        let count = sizes_count.entry(size).or_insert(0);
        *count += 1;
    }
    let smallest = *sizes_count.keys().min().unwrap();
    let smallest_count = *sizes_count.get(&smallest).unwrap();
    (smallest, smallest_count)
}

fn main() {
    let containers = parse(include_str!("day17.txt"));
    let num_150l_containers = PermutationsWithSum::new(&containers, 150).count();
    println!("Number of 150 liter combinations: {}", num_150l_containers);
    let (size_smallest_combination, num_smallest_combinations) = count_smallest(PermutationsWithSum::new(&containers, 150));
    println!("Number of smallest ({}) combination of containers: {}", size_smallest_combination, num_smallest_combinations);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let items = parse(include_str!("day17.txt"));
        assert_eq!(items.len(), 20);
    }

    #[test]
    fn permuting0() {
        let items = parse("");
        let mut it = Permutations::new(&items);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn permuting2() {
        let items = parse("1\n2");
        let mut it = Permutations::new(&items);
        assert_eq!(it.next(), Some(vec![1, 2]));
        assert_eq!(it.next(), Some(vec![1]));
        assert_eq!(it.next(), Some(vec![2]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn permuting3() {
        let items = parse("1\n2\n3");
        let mut it = Permutations::new(&items);
        assert_eq!(it.next(), Some(vec![1, 2, 3]));
        assert_eq!(it.next(), Some(vec![1, 2]));
        assert_eq!(it.next(), Some(vec![1, 3]));
        assert_eq!(it.next(), Some(vec![1]));
        assert_eq!(it.next(), Some(vec![2, 3]));
        assert_eq!(it.next(), Some(vec![2]));
        assert_eq!(it.next(), Some(vec![3]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn permuting4() {
        let items = parse("1\n2\n3\n4");
        let mut it = Permutations::new(&items);
        assert_eq!(it.next(), Some(vec![1, 2, 3, 4]));
        assert_eq!(it.next(), Some(vec![1, 2, 3]));
        assert_eq!(it.next(), Some(vec![1, 2, 4]));
        assert_eq!(it.next(), Some(vec![1, 2]));
        assert_eq!(it.next(), Some(vec![1, 3, 4]));
        assert_eq!(it.next(), Some(vec![1, 3]));
        assert_eq!(it.next(), Some(vec![1, 4]));
        assert_eq!(it.next(), Some(vec![1]));
        assert_eq!(it.next(), Some(vec![2, 3, 4]));
        assert_eq!(it.next(), Some(vec![2, 3]));
        assert_eq!(it.next(), Some(vec![2, 4]));
        assert_eq!(it.next(), Some(vec![2]));
        assert_eq!(it.next(), Some(vec![3, 4]));
        assert_eq!(it.next(), Some(vec![3]));
        assert_eq!(it.next(), Some(vec![4]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn containers() {
        let containers = parse("20\n15\n10\n5\n5");
        let mut it = PermutationsWithSum::new(&containers, 25);
        assert_eq!(it.next(), Some(vec![20, 5]));
        assert_eq!(it.next(), Some(vec![20, 5]));
        assert_eq!(it.next(), Some(vec![15, 10]));
        assert_eq!(it.next(), Some(vec![15, 5, 5]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn counting_smallest() {
        let containers = parse("20\n15\n10\n5\n5");
        let it = PermutationsWithSum::new(&containers, 25);
        assert_eq!(count_smallest(it), (2, 3));
    }
}
