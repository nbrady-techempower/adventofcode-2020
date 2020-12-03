use crate::utils::*;

fn get_trees(input: &Vec<&str>, slope: (usize, usize)) -> i64 {
    let mut right = 0;
    let mut num_trees = 0;
    for i in (slope.1..input.len()).step_by(slope.1) {
        right += slope.0;
        if right > input[i].len() - 1 {
            right = right - input[i].len();
        }
        if input[i].chars().nth(right) == Some('#') {
            num_trees += 1;
        }
    }
    num_trees
}

fn part1(input: &Vec<&str>) {
    println!(" Part 1: {:?}", get_trees(input, (3, 1)));
}

fn part2(input: &Vec<&str>) {
    println!(" Part 2: {:?}", vec![(1,1), (3,1), (5,1), (7,1), (1,2)].iter().fold(1, |mut total, val| total * get_trees(input, *val)));
}

// https://adventofcode.com/2020/day/3
pub fn solve() {
    println!("** Day 3 **");
    let input = read_file("day3-input.txt");
    let input: Vec<&str> = input.split("\n").collect();
    part1(&input);
    part2(&input);
}
