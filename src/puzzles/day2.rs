use crate::*;

fn get_min_max(s: &str) -> (usize, usize) {
    let parts: Vec<usize> = s.split("-").map(|i| i.parse().unwrap_or(0)).collect();
    (parts[0], parts[1])
}

fn part1(input: &Vec<&str>) {
    let mut valid = 0;
    for i in input.iter() {
        let parts: Vec<&str> = i.split(" ").collect();
        let count = parts[2].matches(&parts[1][0..1]).count();
        let (min, max) = get_min_max(parts[0]);
        valid += i32!((min..=max).contains(&count));
    }
    part1!(valid);
}

fn part2(input: &Vec<&str>) {
    let mut valid = 0;
    for i in input.iter() {
        let parts: Vec<&str> = i.split(" ").collect();
        let (min, max) = get_min_max(parts[0]);
        valid += i32!((parts[2][(min - 1)..min] == parts[1][0..1]) ^ (parts[2][(max - 1)..max] == parts[1][0..1]));
    }
    part2!(valid);
}

// https://adventofcode.com/2020/day/2
pub fn solve(input: String) {
    let input: Vec<&str> = input.split("\n").collect();
    part1(&input);
    part2(&input);
}
