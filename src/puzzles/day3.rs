use crate::utils::*;

fn part1(input: &Vec<&str>) {
    let mut right = 0;
    let mut num_trees = 0;
    for i in 1..input.len() {
        right += 3;
        if right > input[i].len() - 1 {
            right = right - input[i].len();
        }
        if input[i].chars().nth(right) == Some('#') {
            num_trees += 1;
        }
    }
    println!(" Part 1: {:?}", num_trees);
}

fn part2(input: &Vec<&str>) {
    let slopes: Vec<(usize, usize)> = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut total_trees: i64 = 1;
    for slope in slopes.iter() {
        let mut num_trees = 0;
        let mut right = 0;
        for i in (slope.1..input.len()).step_by(slope.1) {
            right += slope.0;
            if right > input[i].len() - 1 {
                right = right - input[i].len();
            }
            if input[i].chars().nth(right) == Some('#') {
                num_trees += 1;
            }
        }
        total_trees = total_trees * num_trees;
    }

    println!(" Part 2: {:?}", total_trees);
}

// https://adventofcode.com/2020/day/3
pub fn solve() {
    println!("** Day 3 **");
    let input = read_file("day3-input.txt");
    let input: Vec<&str> = input.split("\n").collect();
    part1(&input);
    part2(&input);
}
