use crate::*;
use std::collections::HashMap;

fn add_mask(mask: String, num: String, part1: bool) -> String {
    let mut to_ret = "".to_string();
    for (u, c) in mask.chars().enumerate() {
        to_ret = match c {
            'X'|'0' if c == 'X' && part1 || c == '0' && !part1 => string!(to_ret, &num[u..u+1]),
            _ => string!(to_ret, c)
        };
    }
    to_ret
}

fn part1(input: Vec<String>) {
    let mut mask = "".to_string();
    let mut hash = HashMap::new();
    for i in input {
        if i.starts_with("mem") {
            let mem = i32!(i[4..i.find("]").unwrap()]);
            let val = i32!(i[i.find("= ").unwrap()+2..]);
            let tmp = format!("{:0>36}", format!("{:b}", val));
            hash.insert(mem, add_mask(mask.to_string(), tmp, true).to_string());
        } else if i.starts_with("mask = ") {
            mask = i[7..].to_string();
        }
    }
    let total = hash.values().fold(0, |acc, v| acc + isize::from_str_radix(v, 2).unwrap());
    part1!(total);
}

fn change_x(v: String) -> Vec<String> {
    let mut to_check: Vec<String> = vec!(v.clone());
    let mut to_ret: Vec<String> = vec!();
    while to_check.len() != 0 {
        if to_check[0].contains('X') {
            to_check.push(to_check[0].replacen('X', "0", 1));
            to_check.push(to_check[0].replacen('X', "1", 1));
            to_check.remove(0);
        } else {
            to_ret.push(to_check[0].clone());
            to_check.remove(0);
        }
    }
    to_ret
}

fn part2(input: Vec<String>) {
    let mut mask = "".to_string();
    let mut hash = HashMap::new();
    for i in input {
        if i.starts_with("mem") {
            let mem = i32!(i[4..i.find("]").unwrap()]);
            let val = i32!(i[i.find("= ").unwrap()+2..]);
            let tmp = add_mask(mask.to_string(), format!("{:0>36}", format!("{:b}", mem)), false);
            for j in change_x(tmp.to_string()) {
                hash.insert(isize::from_str_radix(j.as_str(), 2).unwrap(), val);
            }
        } else {
            mask = i[7..].to_string();
        }
    }
    let total: i64 = hash.values().fold(0, |acc, v| acc + *v as i64);
    part2!(total);
}

// https://adventofcode.com/2020/day/14
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    part1(input.clone());
    part2(input.clone());
}
