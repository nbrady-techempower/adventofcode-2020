use crate::utils::*;

// https://adventofcode.com/2020/day/1
pub fn solve() {
    let input = read_file("day1-input.txt");
    let input: Vec<i64> = input.to_vec_i64();
    for i in 0..(input.len() - 1) {
        for j in i..(input.len() - 2) {
            if input[i] + input[j] == 2020 {
                println!("Part 1: {}", input[i] * input[j]);
            }
            for k in j..(input.len() - 3) {
                if input[i] + input[j] + input[k] == 2020 {
                    println!("Part 2: {}", input[i] * input[j] * input[k]);
                }
            }
        }
    }
}
