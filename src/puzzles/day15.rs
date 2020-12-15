use crate::*;
use std::collections::HashMap;

fn lets_go(input: Vec<i64>, count: i64) -> i64 {
    let mut hash: HashMap<i64, (i64, i64)> = HashMap::new();
    let mut turn: i64 = 0;
    let mut last = 0;
    for i in input {
        turn += 1;
        hash.insert(i, (turn, 0));
        last = i;
    }
    while turn < count {
        turn += 1;
        if let Some(h) = hash.get(&last) {
            if h.1 == 0 {
                last = 0;
                hash.insert(0, (turn, hash.get(&0).unwrap().0));
            } else {
                let last_hash = hash.get(&last).unwrap();
                last = last_hash.0 - last_hash.1;
                if let Some(last_hash) = hash.get(&last) {
                    hash.insert(last, (turn, last_hash.0));
                } else {
                    hash.insert(last, (turn, 0));
                }

            }
        }
    }
    last
}

// https://adventofcode.com/2020/day/15
pub fn solve(input: String) {
    let input: Vec<i64> = input.split(',').map(|c| c.parse::<i64>().unwrap()).collect();
    part1!(lets_go(input.clone(), 2020));
    part2!(lets_go(input.clone(), 30000000));
}
