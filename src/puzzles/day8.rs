use crate::*;
use std::collections::HashSet;

fn part1(input: &Vec<&str>) {
    let mut run = HashSet::new();
    let mut acc = 0;
    let mut i = 0;
    loop {
        let tmp = input[i];
        if run.contains(&i) {
            part1!(acc);
            break;
        }
        run.insert(i);
        if tmp.starts_with("acc") {
            acc += tmp[4..].parse::<i32>().unwrap_or(0);
            i += 1;
        } else if tmp.starts_with("jmp") {
            i = (i as i32 + tmp[4..].parse::<i32>().unwrap_or(0)) as usize;
        } else if tmp.starts_with("nop") {
            i += 1;
        }
    }
}

fn part2(input: &Vec<&str>) {
    let mut run = HashSet::new();
    let mut tried = HashSet::new();
    let mut swapped = false;
    let mut acc = 0;
    let mut i = 0;
    loop {
        if run.contains(&i) {
            i = 0;
            run = HashSet::new();
            swapped = false;
            acc = 0;
        }
        if i == input.len() {
            part2!(acc);
            break;
        }
        let tmp = input[i];
        run.insert(i);
        if tmp.starts_with("acc") {
            acc += tmp[4..].parse::<i32>().unwrap_or(0);
            i += 1;
        } else if tmp.starts_with("jmp") {
            if swapped || tried.contains(&i) {
                i = (i as i32 + tmp[4..].parse::<i32>().unwrap_or(0)) as usize;
            } else {
                tried.insert(i);
                i += 1;
                swapped = true;
            }
        } else if tmp.starts_with("nop") {
            if swapped || tried.contains(&i) {
                i += 1;
            } else {
                tried.insert(i);
                i = (i as i32 + tmp[4..].parse::<i32>().unwrap_or(0)) as usize;
                swapped = true;
            }
        }
    }
}

// https://adventofcode.com/2020/day/8
pub fn solve(input: String) {
    let input: Vec<&str> = input.split("\n").collect();
    part1(&input);
    part2(&input);
}
