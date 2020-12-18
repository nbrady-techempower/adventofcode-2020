use crate::*;

fn get_sum2(s: &str, part: u8) -> i64 {
    let mut parts = s.split(' ').into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut more_to_add = false;
    'top: while parts.len() != 1 {
        if part == 2 && parts.clone().into_iter().filter(|c| c == &"+").collect::<Vec<String>>().len() > 0 {
            more_to_add = true;
        } else if part == 2 {
            more_to_add = false;
        }
        for i in (0..parts.len()).step_by(2) {
            if !["+", "*"].contains(&parts[i+1].as_str()) || (more_to_add && &parts[i+1] == "*") {
                continue;
            }
            let mut new_part = "".to_string();
            if parts[i+1] == "+" {
                new_part = (i64!(parts[i]) + i64!(parts[i+2])).to_string();
            } else if parts[i+1] == "*" {
                new_part = (i64!(parts[i]) * i64!(parts[i+2])).to_string();
            }
            parts.remove(i);
            parts.remove(i);
            parts.remove(i);
            parts.insert(i, new_part);
            continue 'top;
        }
    }
    i64!(parts[0])
}

fn lets_go(input: Vec<String>, part: u8) {
    let mut to_ret = 0;

    for i in input {
        let mut s = i.clone();
        while s.contains('(') {
            let (mut idx1, mut idx2) = (0, 0);
            for (i, c) in s.chars().enumerate() {
                if c == '(' {
                    idx1 = i;
                } else if c == ')' {
                    idx2 = i;
                    break;
                }
            }
            s = string!(&s[0..idx1], get_sum2(&s[idx1+1..idx2], part), &s[idx2+1..]);
        }
        to_ret += get_sum2(&s[0..], part)
    }
    println!(" Part {}: {}", part, to_ret);
}

// https://adventofcode.com/2020/day/18
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    lets_go(input.clone(), 1);
    lets_go(input.clone(), 2);
}
