pub fn pairs<T>(vec: Vec<T>) -> Pairs<T> {
    Pairs::new(vec)
}

#[derive(Debug)]
pub struct Pairs<T> {
    vec: Vec<T>,
    outer_idx: usize,
    inner_idx: usize
}

impl<T> Pairs<T> {
    fn new(vec: Vec<T>) -> Self {
        Pairs {
            vec,
            outer_idx: 0,
            inner_idx: 0
        }
    }
}

impl<T> Iterator for Pairs<T>
    where T: Copy
{
    type Item = (T, T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.vec.len() < 2 {
            return None;
        }
        self.inner_idx += 1;
        // Move the outer index and reset the inner index
        if self.inner_idx >= self.vec.len() {
            self.outer_idx += 1;
            self.inner_idx = self.outer_idx + 1;
        }
        // Once the outer index is exhausted, we done here
        if self.outer_idx >= self.vec.len() - 1 {
            return None;
        }
        Some((self.vec[self.outer_idx], self.vec[self.inner_idx]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_pairs() {
        let empty: Vec<i32> = vec!();
        assert_eq!(pairs(empty).next(), None);
        assert_eq!(pairs(vec!(0)).next(), None);
    }

    #[test]
    fn one_pair() {
        let vec: Vec<i32> = vec!(1, 2);
        assert_eq!(pairs(vec).next(), Some((1, 2)));
    }


    #[test]
    fn multiple_pairs() {
        let vec: Vec<i32> = vec!(1, 2, 3, 4, 5);
        let mut count = 0;
        for p in pairs(vec.clone()) {
            match count {
                0 => assert_eq!(p, (1, 2)),
                1 => assert_eq!(p, (1, 3)),
                2 => assert_eq!(p, (1, 4)),
                3 => assert_eq!(p, (1, 5)),
                4 => assert_eq!(p, (2, 3)),
                5 => assert_eq!(p, (2, 4)),
                6 => assert_eq!(p, (2, 5)),
                7 => assert_eq!(p, (3, 4)),
                8 => assert_eq!(p, (3, 5)),
                9 => assert_eq!(p, (4, 5)),
                _ => ()
            }
            count += 1;
        }
        assert_eq!(pairs(vec.clone()).count(), 10);
    }
}
