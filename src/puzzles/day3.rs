use crate::*;

fn get_trees(input: &Vec<&str>, slope: (usize, usize)) -> i64 {
    let (mut right, mut num_trees) = (0, 0);
    for i in (slope.1..input.len()).step_by(slope.1) {
        right = (right + slope.0) % input[i].len();
        num_trees += (input[i].chars().nth(right) == Some('#')) as i64;
    }
    num_trees
}

// https://adventofcode.com/2020/day/3
pub fn solve(input: String) {
    let input: Vec<&str> = input.split("\n").collect();
    part1!(get_trees(&input, (3, 1)));
    part2!(vec![(1,1), (3,1), (5,1), (7,1), (1,2)].iter().fold(1, |total, val| total * get_trees(&input, *val)));
}
