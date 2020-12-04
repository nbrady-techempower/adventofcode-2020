pub trait IntUtils {
    fn rev(&self) -> Self;
}

macro_rules! int_utils {
    ($($type: ty),*) => {
        $(
            impl IntUtils for $type {
                fn rev(&self) -> Self {
                    self.to_string().chars().rev().collect::<String>().parse::<$type>().unwrap_or(0)
                }
            }
        )*
   }
}

int_utils!(i32, i64);

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn reverse_for_int_works() {
        assert_eq!(321_i32.rev(), 123);
        assert_eq!(32100_i32.rev(), 123);
        assert_eq!(0_i32.rev(), 0);
        assert_eq!(1234567890_i64.rev(), 987654321);
    }
}