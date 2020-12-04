use crate::*;
use crate::utils::*;

fn part1(input: &Vec<i64>) {
    for i in 0..(input.len() - 1) {
        for j in (i+1)..input.len() {
            if input[i] + input[j] == 2020 {
                part1!(input[i] * input[j]);
            }
        }
    }
}

fn part2(input: &Vec<i64>) {
    for i in 0..(input.len() - 2) {
        for j in (i+1)..(input.len() - 1) {
            for k in (j+1)..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    part2!(input[i] * input[j] * input[k]);
                }
            }
        }
    }
}

// https://adventofcode.com/2020/day/1
pub fn solve(input: String) {
    let input: Vec<i64> = input.to_vec_i64();
    part1(&input);
    part2(&input);
}
