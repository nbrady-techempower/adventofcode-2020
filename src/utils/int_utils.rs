#[macro_export]
macro_rules! i32 {
    ($thing:expr) => {
        $thing.to_i32()
    }
}

#[macro_export]
macro_rules! i64 {
    ($thing:expr) => {
        $thing.to_i64()
    }
}

#[macro_export]
macro_rules! usize {
    ($thing:expr) => {
        $thing.to_usize()
    }
}

pub trait IntUtils {
    // Reverses a number, retaining signs
    fn rev(&self) -> Self;
    fn to_i32(&self) -> i32;
    fn to_i64(&self) -> i64;
}

macro_rules! int_utils {
    ($($type: ty),*) => {
        $(
            impl IntUtils for $type {
                fn rev(&self) -> Self {
                    let is_negative = *self < 0;
                    let to_ret = self.to_string().chars().rev().collect::<String>().parse::<$type>().unwrap_or(0);
                    match is_negative {
                        true => to_ret * -1,
                        false => to_ret
                    }
                }

                fn to_i32(&self) -> i32 {
                    *self as i32
                }

                fn to_i64(&self) -> i64 {
                    *self as i64
                }
            }
        )*
   }
}

int_utils!(i32, i64);

pub trait BoolUtils {
    fn to_i32(&self) -> i32;
    fn to_i64(&self) -> i64;
    fn to_usize(&self) -> usize;
}

impl BoolUtils for bool {
    fn to_i32(&self) -> i32 {
        *self as i32
    }

    fn to_i64(&self) -> i64 {
        *self as i64
    }

    fn to_usize(&self) -> usize {
        *self as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn reverse_for_int_works() {
        assert_eq!(321_i32.rev(), 123);
        assert_eq!(32100_i32.rev(), 123);
        assert_eq!(0_i32.rev(), 0);
        assert_eq!(1234567890_i64.rev(), 987654321);
        assert_eq!(-1234567890_i64.rev(), -987654321);
    }
}