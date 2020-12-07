use crate::*;
use std::collections::HashSet;

// O(my) this is not good lol
fn part1(input: &Vec<&str>) {
    let mut direct: Vec<&str> = vec!();
    let mut new_direct: Vec<&str> = vec!();
    let mut all_bags = HashSet::new();
    for i in input {
        if i.contains("shiny gold") {
            let bag = &i[0..i.find(" bag").unwrap()];
            direct.push(bag);
            all_bags.insert(bag);
        }
    }
    let mut found = true;
    while found {
        found = false;
        for i in input {
            for d in direct.clone() {
                if i[1..].contains(d) {
                    let bag = &i[0..i.find(" bag").unwrap()];
                    new_direct.push(bag);
                    all_bags.insert(bag);
                    found = true;
                }
            }
        }
        direct = new_direct.clone();
        new_direct = vec!();
    }

    part1!(all_bags.len());
}

fn get_bags<'a>(input: &'a Vec<&str>, bag: (i32, &str)) -> Vec<(i32, &'a str)> {
    let mut to_ret: Vec<(i32, &str)> = vec!();


    for i in input {
        if i.find(bag.1) == Some(0) {
            if i.contains("contain no other") {
                return to_ret;
            }

            let tmp = i.split(",").collect::<Vec<&str>>();
            let s = tmp[0];
            let s = &s[(s.find("contain ").unwrap() + 8)..];
            let space = s.find(" ").unwrap();

            let bag = &s[space..s.find(" bag").unwrap()].trim();
            let num = s[0..space].parse::<i32>().unwrap();
            to_ret.push((num, bag));
            for t in 1..tmp.len() {
                let s = tmp[t].trim();
                let space = s.find(" ").unwrap();
                let bag = &s[space..s.find(" bag").unwrap()].trim();
                let num = s[0..space].parse::<i32>().unwrap();
                to_ret.push((num, bag));
            }
            return to_ret;
        }
    }
    to_ret
}

fn part2(input: &Vec<&str>) {
    let mut to_ret = 0;
    let mut remaining: Vec<(i32, &str)> = vec![(1, "shiny gold")];
    while remaining.len() > 0 {
        to_ret += remaining[0].0;
        for b in get_bags(&input,remaining[0]) {
            remaining.push((remaining[0].0 * b.0, b.1));
        }
        remaining.remove(0);
    }

    part2!(to_ret - 1);
}

// https://adventofcode.com/2020/day/7
pub fn solve(input: String) {
    let input: Vec<&str> = input.split("\n").collect();
    part1(&input);
    part2(&input);
}
