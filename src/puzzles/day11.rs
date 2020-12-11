use crate::*;

fn num_seats(input: Vec<String>, i: usize, j: usize) -> i32 {
    let mut to_ret = 0;
    if i > 0 {
        if let Some(s) = input.get(i - 1) {
            if j > 0 {
                if let Some(c) = s.chars().nth(j - 1) {
                    if c == '#' {
                        to_ret += 1;
                    }
                }
            }
            if let Some(c) = s.chars().nth(j) {
                if c == '#' {
                    to_ret += 1;
                }
            }
            if let Some(c) = s.chars().nth(j + 1) {
                if c == '#' {
                    to_ret += 1;
                }
            }
        }
    }
    if let Some(s) = input.get(i) {
        if j > 0 {
            if let Some(c) = s.chars().nth(j - 1) {
                if c == '#' {
                    to_ret += 1;
                }
            }
        }

        if let Some(c) = s.chars().nth(j+1) {
            if c == '#' {
                to_ret += 1;
            }
        }
    }
    if let Some(s) = input.get(i+1) {
        if j > 0 {
            if let Some(c) = s.chars().nth(j - 1) {
                if c == '#' {
                    to_ret += 1;
                }
            }
        }
        if let Some(c) = s.chars().nth(j) {
            if c == '#' {
                to_ret += 1;
            }
        }
        if let Some(c) = s.chars().nth(j+1) {
            if c == '#' {
                to_ret += 1;
            }
        }
    }
    to_ret
}

fn num_seats2(input: Vec<String>, i: usize, j: usize) -> i32 {
    let mut to_ret = 0;
    let mut tmp = 0;
    let mut tmp2 = 0;

    // top
    if i > 0 {
        tmp = i - 1;
        loop {
            if let Some(s) = input.get(tmp) {
                if let Some(c) = s.chars().nth(j) {
                    if c == '#' {
                        to_ret += 1;
                        break;
                    }
                    if c == 'L' {
                        break;
                    }
                }
            }
            if tmp == 0 {
                break;
            }
            tmp -= 1;
        }
    }

    // bottom
    let mut tmp = i+1;
    loop {
        if let Some(s) = input.get(tmp) {
            if let Some(c) = s.chars().nth(j) {
                if c == '#' {
                    to_ret += 1;
                    break;
                }
                if c == 'L' {
                    break;
                }
            }
        } else {
            break;
        }
        tmp += 1;
    }

    // right
    let mut tmp = j+1;
    loop {
        if let Some(s) = input.get(i) {
            if let Some(c) = s.chars().nth(tmp) {
                if c == '#' {
                    to_ret += 1;
                    break;
                }
                if c == 'L' {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        tmp += 1;
    }

    // left
    if j > 0 {
        tmp = j - 1;
        loop {
            if let Some(s) = input.get(i) {
                if let Some(c) = s.chars().nth(tmp) {
                    if c == '#' {
                        to_ret += 1;
                        break;
                    }
                    if c == 'L' {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
            if tmp == 0 {
                break;
            }
            tmp -= 1;
        }
    }

    // up & left
    if i > 0 && j > 0 {
        tmp = i - 1;
        tmp2 = j - 1;
        loop {
            if let Some(s) = input.get(tmp) {
                if let Some(c) = s.chars().nth(tmp2) {
                    if c == '#' {
                        to_ret += 1;
                        break;
                    }
                    if c == 'L' {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
            if tmp == 0 || tmp2 == 0 {
                break;
            }
            tmp -= 1;
            tmp2 -= 1;
        }
    }

    // up & right
    if i > 0 {
        tmp2 = j + 1;
        tmp = i - 1;
        loop {
            if let Some(s) = input.get(tmp) {
                if let Some(c) = s.chars().nth(tmp2) {
                    if c == '#' {
                        to_ret += 1;
                        break;
                    }
                    if c == 'L' {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
            if tmp == 0 {
                break;
            }
            tmp -= 1;
            tmp2 += 1;
        }
    }

    // down & left
    if j > 0 {
        tmp = i + 1;
        tmp2 = j - 1;
        loop {
            if let Some(s) = input.get(tmp) {
                if let Some(c) = s.chars().nth(tmp2) {
                    if c == '#' {
                        to_ret += 1;
                        break;
                    }
                    if c == 'L' {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
            if tmp2 == 0 {
                break;
            }
            tmp += 1;
            tmp2 -= 1;
        }
    }

    // down & right
    let mut tmp = i+1;
    let mut tmp2 = j+1;
    loop {
        if let Some(s) = input.get(tmp) {
            if let Some(c) = s.chars().nth(tmp2) {
                if c == '#' {
                    to_ret += 1;
                    break;
                }
                if c == 'L' {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        if tmp2 == 0 {
            break;
        }
        tmp += 1;
        tmp2 += 1;
    }
    to_ret
}


fn part1(input: Vec<String>, max_seats: i32, seats: fn(Vec<String>, usize, usize) -> i32) {
    let mut old = input.clone();
    let mut new: Vec<String> = vec!();
    let mut found = false;
    let mut count = 0;

    let mut test = 0;
    while !found {
        test += 1;
        for i in 0..old.len() {
            let mut s = "".to_string();
            for j in 0..old[i].len() {
                let mut new = '.';
                let c = old[i].chars().nth(j).unwrap();
                let seats = seats(old.clone(), i, j);
                if c == 'L' && seats == 0 {
                    new = '#';
                } else if c == 'L' {
                    new = 'L';
                }
                if c == '#' && seats >= max_seats {
                    new = 'L';
                } else if c == '#' {
                    new = '#';
                }
                s = s.to_string() + new.to_string().as_str();
            }
            new.push(s);
        }

        found = true;
        for i in 0..old.len() {
            if old[i] != new[i] {
                found = false;
            }
        }

        if !found {
            old = new.clone();
            new = vec!();
            count = 0;
        } else {
            for i in 0..old.len() {
                count += old[i].matches('#').count();
            }
        }

    }

    part1!(count);
}


// https://adventofcode.com/2020/day/11
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    part1(input.clone(), 4, num_seats);
    part1(input.clone(), 5, num_seats2);
}
