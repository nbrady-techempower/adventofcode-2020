// I altered the input of this puzzle to remove the name labels
// and add my own ticket to the list of nearby tickets for easier parsing

use crate::*;
use std::ops::{Range, RangeInclusive};

type Ranges = Vec<RangeInclusive<i32>>;
type Tickets = Vec<Vec<i32>>;

fn part1(input: Vec<String>) -> (Ranges, Tickets) {
    let mut ranges: Ranges = vec!();
    let mut to_ret = 0;
    for i in &input {
        if i.starts_with("nearby tickets") {
            break;
        }
        if i.len() == 0 {
            continue;
        }
        let nums = i.split(" or ").collect::<Vec<&str>>();
        let r1 = nums[0].split("-").collect::<Vec<&str>>();
        let r2 = nums[1].split("-").collect::<Vec<&str>>();
        ranges.push(i32!(r1[0])..=i32!(r1[1]));
        ranges.push(i32!(r2[0])..=i32!(r2[1]));
    }

    let mut tickets = false;
    let mut good_tickets: Tickets = vec!();
    for i in &input {
        if i.starts_with("nearby tickets") {
            tickets = true;
            continue;
        }
        if tickets {
            let nums = i.split(",").map(|e| i32!(e)).collect::<Vec<i32>>();
            let mut good_ticket = true;
            for num in &nums {
                let mut found = false;
                for range in ranges.clone() {
                    if range.contains(&num) {
                        found = true;
                    }
                }
                if !found {
                    good_ticket = false;
                    to_ret += num;
                }
            }
            if good_ticket {
                good_tickets.push(nums);
            }
        }
    }
    part1!(to_ret);
    (ranges, good_tickets)
}

fn part2(input: Vec<String>) {
    let (ranges, all) = part1(input.clone());
    let mut col_ranges: Tickets = vec!();

    // Separate tickets into columns
    let mut cols: Vec<Vec<i32>> = vec!();
    for i in 0..all[0].len() {
        cols.push(vec!());
        col_ranges.push(vec!());
        for j in 0..all.len() {
            cols[i].push(all[j][i]);
        }
    }

    // Match cols to the ranges they fit in
    for (idx, c) in cols.iter().enumerate() {
        for i in (0..ranges.len()).step_by(2) {
            if c.iter().all(|e| ranges[i].contains(e) || ranges[i+1].contains(e))
            {
                col_ranges[idx].push((i/2) as i32);
            }
        }
    }

    let mut indexes_for_depart: Vec<usize> = vec!();
    while indexes_for_depart.len() < 6 || col_ranges.iter().any(|c| c.len() > 0) {
        let mut idx_to_rm = 0;
        // Find the vec with only 1 item
        for col_idx in 0..col_ranges.len() {
            if col_ranges[col_idx].len() == 1 {
                idx_to_rm = col_ranges[col_idx].pop().unwrap();
                // if the col range is 0..=5 it's one of our departs. keep the col idx
                if (0..=5).contains(&idx_to_rm) {
                  indexes_for_depart.push(col_idx);
                }
            }
        }
        // Remove it from the other vecs
        for col_idx in 0..col_ranges.len() {
            if let Some(idx) = col_ranges[col_idx].iter().position(|i| i == &idx_to_rm) {
                col_ranges[col_idx].remove(idx);
            }
        }
    }
    part1!(indexes_for_depart.iter().fold(1, |acc: i64, i| acc * i64!(all[0][*i as usize])));
}

// https://adventofcode.com/2020/day/16
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    part2(input.clone());
}
