use crate::*;

fn part1(input: Vec<i64>) {
    let mut one = 0;
    let mut three = 0;

    for i in 0..(input.len() - 1) {
        if input[i] + 1 == input[i + 1] {
            one += 1;
        } else if input[i] + 3 == input[i+1] {
            three += 1;
        }
    }
    part1!(one * three);
}

fn count(input: &Vec<i64>, i: usize) -> i32 {
    let mut to_ret = 1;
    if i == input.len() - 2 {
        return 0;
    }
    for k in i..input.len() - 2 {
        if input[k] + 3 >= input[k + 2] {
            let mut tmp = input.clone();
            tmp.remove(k+1);
            to_ret += count(&tmp, k);
        }
    }
    to_ret
}

fn part2(input: Vec<i64>) {
    let mut to_ret: i64 = 1;
    let mut start= 0;
    for i in 0..input.len() - 2 {
        if input[i] + 3 == input[i+1] && i - start > 10 {
            to_ret *= i64!(count(&input[start..i+2].to_vec(), 0));
            start = i;
        }
    }
    to_ret *= i64!(count(&input[start..].to_vec(), 0));
    part2!(to_ret);
}

// https://adventofcode.com/2020/day/10
pub fn solve(input: String) {
    let mut input: Vec<i64> = input.to_vec_i64();
    input.push(0);
    input.sort();
    input.push(input[input.len() - 1] + 3);

    part1(input.clone());
    part2(input.clone());
}
