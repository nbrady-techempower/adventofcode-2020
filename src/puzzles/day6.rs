use crate::*;

fn part1(input: &String) {
    let mut s = "".to_string();
    let mut to_ret = 0;
    for b in input.to_blocks() {
        for i in b {
            s.push_str(i);
        }
        let mut tmp = s.chars().into_iter().collect::<Vec<char>>();
        tmp.sort();
        tmp.dedup();
        to_ret += tmp.len();
        s = "".to_string();
    }
    part1!(to_ret);
}

fn part2(input: &String) {
    let mut s = "".to_string();
    let mut to_ret = 0;
    let mut num_people = 0;
    for b in input.to_blocks() {
        for i in b {
            num_people += 1;
            s.push_str(i);
        }
        let mut tmp = s.chars().into_iter().collect::<Vec<char>>();
        tmp.sort();
        tmp.dedup();
        for c in tmp {
            if s.matches(c).count() == num_people {
                to_ret += 1;
            }
        }
        num_people = 0;
        s = "".to_string();
    }
    part2!(to_ret);
}

// https://adventofcode.com/2020/day/6
pub fn solve(input: String) {
    part1(&input);
    part2(&input);
}
