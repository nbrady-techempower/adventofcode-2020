// Implementations that can be used across String and &str and built with macros
pub trait StrUtilsCommon {
    fn contains_any(&self, patterns: Vec<&str>) -> bool;
    fn contains_all(&self, patterns: Vec<&str>) -> bool;
    fn is_palindrome(&self) -> bool;
    fn swap(&self, idx1: usize, idx2: usize) -> String;
}

macro_rules! str_utils {
    ($($type: ty),*) => {
        $(
            impl StrUtilsCommon for $type {
                fn contains_any(&self, patterns: Vec<&str>) -> bool {
                    for p in patterns {
                        if self.contains(p) {
                            return true;
                        }
                    }
                    false
                }

                fn contains_all(&self, patterns: Vec<&str>) -> bool {
                    for p in patterns {
                        if !self.contains(p) {
                            return false;
                        }
                    }
                    true
                }

                fn is_palindrome(&self) -> bool {
                   self.to_string() == self.chars().rev().collect::<String>()
                }

                fn swap(&self, idx1: usize, idx2: usize) -> String {
                    let mut tmp: Vec<char> = self.chars().into_iter().collect::<Vec<char>>();
                    let hold = tmp[idx2];
                    tmp[idx2] = tmp[idx1];
                    tmp[idx1] = hold;
                    tmp.into_iter().collect::<String>()
                }
            }
        )*
   }
}

str_utils!(String, &str);


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn palindrome_works() {
        assert_eq!("abccba".is_palindrome(), true);
        assert_eq!("aba".is_palindrome(), true);
        assert_eq!("ab a".is_palindrome(), false);
        assert_eq!("".is_palindrome(), true);
        assert_eq!("漢".is_palindrome(), true);
        assert_eq!("abccba".to_string().is_palindrome(), true);
        assert_eq!(12344321.to_string().is_palindrome(), true);
    }

    #[test]
    fn swap_works() {
        assert_eq!("dog".swap(0,2), "god");
        assert_eq!("1".swap(0, 0), "1");
    }

    #[test]
    #[should_panic]
    fn swap_should_panic() {
        assert_eq!("1".swap(0, 1), "1");
    }
}