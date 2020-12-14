use crate::*;
use std::collections::HashMap;

fn add_mask(mask: String, num: String) -> String {
    let mut to_ret = "".to_string();
    for (u, c) in mask.chars().enumerate() {
        if c == 'X' {
            to_ret = to_ret.to_string() + num.chars().nth(u).unwrap().to_string().as_str();
        } else {
            to_ret = to_ret.to_string() + c.to_string().as_str();
        }
    }
    to_ret
}

fn add_mask2(mask: String, num: String) -> String {
    let mut to_ret = "".to_string();
    for (u, c) in mask.chars().enumerate() {
        if c == '0' {
            to_ret = to_ret.to_string() + num.chars().nth(u).unwrap().to_string().as_str();
        } else {
            to_ret = to_ret.to_string() + c.to_string().as_str();
        }
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
            hash.insert(mem, add_mask(mask.to_string(), tmp).to_string());
        }
        else if i.starts_with("mask = ") {
            mask = i[7..].to_string();
        }
    }

    let mut total = 0;
    for v in hash.values() {
        let num = isize::from_str_radix(v, 2).unwrap();
        total += num;
    }
    part1!(total);

}

fn change_x(v: String) -> Vec<String> {
    let mut to_check: Vec<String> = vec!(v.clone());
    let mut to_ret: Vec<String> = vec!();
    'top: loop {
        if to_check.len() == 0 {
            break;
        }
        for idx in 0..to_check.len() {
            if to_check[idx].contains('X') {
                let tmp = to_check[idx].clone().replacen('X', "0", 1);
                let tmp2 = to_check[idx].clone().replacen('X', "1", 1);
                to_check.remove(idx);
                to_check.push(tmp.clone());
                to_check.push(tmp2.clone());
            } else {
                to_ret.push(to_check[idx].to_string());
                to_check.remove(idx);
            }
            continue 'top;
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
            let tmp = format!("{:0>36}", format!("{:b}", mem));
            let tmp = add_mask2(mask.to_string(), tmp);
            for j in change_x(tmp.to_string()) {
                let num = isize::from_str_radix(j.as_str(), 2).unwrap();
                hash.insert(num, val);
            }
        }
        else if i.starts_with("mask = ") {
            mask = i[7..].to_string();
        }
    }

    let mut total: i64 = 0;
    for v in hash.values() {
            total += *v as i64;
        }
    part1!(total);
}

// https://adventofcode.com/2020/day/14
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    part1(input.clone());
    part2(input.clone());
}
