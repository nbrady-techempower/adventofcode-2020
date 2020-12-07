use std::fs::read_to_string;

#[macro_export]
macro_rules! part1 {
    ($($x: expr),*) => {{
        print!(" Part 1: ");
        $(print!("{:?}", $x);)*
        println!();
    }}
}

#[macro_export]
macro_rules! part2 {
    ($($x: expr),*) => {{
        print!(" Part 2: ");
        $(print!("{:?}", $x);)*
        println!();
    }}
}

pub fn read_file(p: &str) -> String {
    read_to_string("C:/Development/aoc-2020/src/puzzles/".to_owned() + p)
        .unwrap_or("".to_string())
}

pub trait InputUtils {
    fn to_vec_i64(&self) -> Vec<i64>;
    fn to_blocks(&self) -> Vec<Vec<&str>>;
}

impl InputUtils for String {
    fn to_vec_i64(&self) -> Vec<i64> {
        self.split('\n').map(|i| i.parse::<i64>().unwrap_or(0)).collect()
    }

    fn to_blocks(&self) -> Vec<Vec<&str>> {
        let input: Vec<&str> = self.split("\n").collect();
        let mut to_ret: Vec<Vec<&str>> = vec!(vec!());
        let mut tmp: Vec<&str> = vec!();
        for i in input {
            if i.len() == 0 {
                to_ret.push(tmp.clone());
                tmp = vec!();
            } else {
                tmp.push(i);
            }
        }
        to_ret
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::utils::InputUtils;

    #[test]
    fn str_to_vec_i64_works() {
        let test = "123
456
789
023".to_string();
        assert_eq!(test.to_vec_i64(), vec![123, 456, 789, 23]);
        assert_eq!("".to_string().to_vec_i64(), vec![0]);
    }
}
