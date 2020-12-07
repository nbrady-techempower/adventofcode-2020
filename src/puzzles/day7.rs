use crate::*;
use std::collections::HashSet;
use regex::Regex;

type Bag = (i32, String);

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

fn get_bags(input: &Vec<&str>, bag: Bag) -> Vec<Bag> {
    let mut to_ret: Vec<Bag> = vec!();

    for i in input {
        if i.find(&bag.1) == Some(0) {
            if i.contains("contain no other") {
                return to_ret;
            }
            let tmp = i.split(",").collect::<Vec<&str>>();
            for t in tmp {
                let re = Regex::new(r"(\d+) (.*) bag").unwrap();
                let cap = re.captures(t).unwrap();
                to_ret.push((cap[1].parse::<i32>().unwrap(), cap[2].to_string()));
            }
            return to_ret;
        }
    }
    to_ret
}

fn part2(input: &Vec<&str>) {
    let mut to_ret = 0;
    let mut remaining: Vec<Bag> = vec![(1, "shiny gold".to_string())];
    while remaining.len() > 0 {
        to_ret += remaining[0].0;
        for b in get_bags(&input,remaining[0].clone()) {
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
