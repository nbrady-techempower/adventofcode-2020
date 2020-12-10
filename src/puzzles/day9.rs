use crate::*;

fn part1(input: Vec<i64>) {
    let (mut i, mut found) = (24, true);
    while found {
        found = false;
        i += 1;
        for (j, k) in pairs(input[(i-25)..i].to_vec()) {
            if j + k == input[i] {
                found = true;
                break;
            }
        }
    }
    part1!(input[i]);
}

fn part2(input: Vec<i64>) {
    let (mut i, mut total, mut found) = (0, 0, vec!());
    let find = 20874512;
    while total != find {
        found = vec!();
        total = 0;
        i += 1;
        for j in input[i..].iter() {
            total += j;
            found.push(*j);
            if total == find {
                break;
            }
        }
    }
    part2!(found.iter().min().unwrap() + found.iter().max().unwrap());
}

// https://adventofcode.com/2020/day/9
pub fn solve(input: String) {
    let input: Vec<i64> = input.to_vec_i64();
    part1(input.clone());
    part2(input.clone());
}
