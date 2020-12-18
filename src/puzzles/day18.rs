use crate::*;
use std::collections::HashMap;

fn get_sum(s: &str) -> i64 {
    let mut to_ret = 0;
    let mut is_val1 = true;
    let mut val1 = "".to_string();
    let mut op = '+';
    for c in s.chars() {
        match c {
            ' ' => {
              if val1.len() == 0 {
                continue;
              }
              if is_val1 {
                  is_val1 = false;
                  to_ret = i64!(val1);
              } else {
                  if op == '+' {
                      to_ret += i64!(val1);
                  } else {
                      to_ret *= i64!(val1);
                  }
              }
                val1 = "".to_string();
            },
            '+'|'*' => op = c,
            _ => {
                val1 = string!(val1, c);
            }
        }
    }
    if op == '+' {
        to_ret += i64!(val1);
    } else {
        to_ret *= i64!(val1);
    }
    to_ret
}

fn get_sum2(s: &str) -> i64 {
    let mut parts = s.split(' ').into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
    'top: loop {
        let tmp = parts.clone();
        if tmp.into_iter().filter(|c| c == &"+").collect::<Vec<String>>().len() == 0 {
            break;
        }
        for i in (0..parts.len()).step_by(2) {
            if parts[i+1] == "+" {
                let new_part = (i64!(parts[i]) + i64!(parts[i+2])).to_string();
                parts.remove(i);
                parts.remove(i);
                parts.remove(i);
                parts.insert(i, new_part);
                continue 'top;
            }
        }
    }
    parts.into_iter().filter(|c| c != &"*").fold(1, |acc, c| acc * i64!(c))
}

fn part1(input: Vec<String>) {
    let mut to_ret = 0;

    for i in input {
        let mut s = i.clone();
        while s.contains('(') {
            let mut idx1 = 0;
            let mut idx2 = 0;
            for (i, c) in s.chars().enumerate() {
                if c == '(' {
                    idx1 = i;
                }
                if c == ')' {
                    idx2 = i;
                    break;
                }
            }
            s = string!(&s[0..idx1], get_sum(&s[idx1+1..idx2]), &s[idx2+1..]);
        }
        to_ret += get_sum(&s[0..])
    }
    part1!(to_ret);
}

fn part2(input: Vec<String>) {
    let mut to_ret = 0;

    for i in input {
        let mut s = i.clone();
        while s.contains('(') {
            let mut idx1 = 0;
            let mut idx2 = 0;
            for (i, c) in s.chars().enumerate() {
                if c == '(' {
                    idx1 = i;
                }
                if c == ')' {
                    idx2 = i;
                    break;
                }
            }
            s = string!(&s[0..idx1], get_sum2(&s[idx1+1..idx2]), &s[idx2+1..]);
        }
        to_ret += get_sum2(&s[0..])
    }
    part2!(to_ret);
}


// https://adventofcode.com/2020/day/18
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    part1(input.clone());
    part2(input.clone());
}
