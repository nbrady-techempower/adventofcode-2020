// Implementations that can be used across String and &str and built with macros
pub trait StrUtilsCommon {
    fn contains_any(&self, patterns: Vec<&str>) -> bool;
    fn contains_all(&self, patterns: Vec<&str>) -> bool;
    fn is_palindrome(&self) -> bool;
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
            }
        )*
   }
}

str_utils!(String, &str);

// pub trait StrUtils {
//     fn is_palindrome(&self) -> bool;
// }
//
// impl StrUtils for &str {
//     fn is_palindrome(&self) -> bool {
//         self.to_string().to_string() == self.chars().rev().collect::<String>()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn palindrome_works() {
        assert_eq!("abccba".is_palindrome(), true);
        assert_eq!("aba".is_palindrome(), true);
        assert_eq!("ab a".is_palindrome(), false);
        assert_eq!("".is_palindrome(), true);
        assert_eq!("abccba".to_string().is_palindrome(), true);
        assert_eq!(12344321.to_string().is_palindrome(), true);
    }
}
