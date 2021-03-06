pub struct Permutations<T> {
    data: Vec<T>,
    n: usize,
    c: Vec<usize>,
}

impl<T: Clone> Permutations<T> {
    pub fn new(data: Vec<T>) -> Permutations<T> {
        let len = data.len();
        Permutations {
            data: data,
            n: !0,
            c: vec![0; len],
        }
    }
}

impl<T> Permutations<T> {
    pub fn next_permutation(&mut self) -> Option<&[T]> {
        if self.n == !0 {
            self.n = 0;
            Some(&self.data)
        } else {
            while self.n < self.data.len() - 1 {
                if self.c[self.n] <= self.n {
                    let j = if self.n % 2 == 0 { self.c[self.n] } else { 0 };
                    self.data.swap(j, self.n + 1);
                    self.c[self.n] += 1;
                    self.n = 0;
                    return Some(&self.data);
                } else {
                    self.c[self.n] = 0;
                    self.n += 1;
                }
            }
            None
        }
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        self.next_permutation().map(|d| d.to_owned())
    }
}

pub trait PermutationExt<T> {
    fn permutations(&self) -> Permutations<T>;
}

impl<T: Clone> PermutationExt<T> for Vec<T> {
    fn permutations(&self) -> Permutations<T> {
        Permutations::new(self.clone())
    }
}

impl<T: Clone> PermutationExt<T> for [T] {
    fn permutations(&self) -> Permutations<T> {
        Permutations::new(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute() {
        let data = [1, 2, 3];
        let mut permutations = data.permutations();
        assert_eq!(permutations.next(), Some(vec![1, 2, 3]));
        assert_eq!(permutations.next(), Some(vec![2, 1, 3]));
        assert_eq!(permutations.next(), Some(vec![3, 1, 2]));
        assert_eq!(permutations.next(), Some(vec![1, 3, 2]));
        assert_eq!(permutations.next(), Some(vec![2, 3, 1]));
        assert_eq!(permutations.next(), Some(vec![3, 2, 1]));
        assert_eq!(permutations.next(), None);
    }
}
