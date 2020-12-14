use crate::*;

fn part1(input: Vec<String>) {
    let time = i32!(input[0].clone());
    let ids = input[1].split(",").collect::<Vec<&str>>();
    let mut min = 1000000;
    let mut wait = 0;
    for i in ids {
        if i == "x" {
            continue;
        }

        let num = i32!(i);
        for n in (0..).step_by(num as usize) {
            if n >= time {
                let diff = n - time;
                if diff < min {
                    min = diff;
                    wait = num * diff;
                }
                break;
            }
        }
    }
    part1!(wait);

}

fn part2(_: Vec<String>) {
    // the biggest id in our input is 823 in the 19th index,
    // so we can start at some multiple of 823, add 804 and step by 823
    // AoC gives us a hint that we'll be over a 100 trillions

    // I put in a guess of 148140000000804 to get a hint again from AoC -- low
    // I put in a guess of 288050000000804 to get a hint again from Aoc -- low
    // I put in a guess of 411500000000804 to get a hint again from Aoc -- low
    // answer brute forced overnight 1058443396696792
    let mut timestamp: i64 = 411500000000804;
    loop {
        timestamp += 823;
        // just something to see how fast we're iterating
        if timestamp % 823_000_000_000 == 804 {
            part2!(timestamp);
        }
        // Removed 823 check since it will always be true
        if
            ((timestamp + 50) % 443 == 0) &&
            (timestamp % 19 == 0) &&
            ((timestamp + 9) % 41 == 0) &&
            ((timestamp + 27) % 23 == 0) &&
            ((timestamp + 36) % 17 == 0) &&
            ((timestamp + 48) % 29 == 0) &&
            ((timestamp + 56) % 37 == 0) &&
            ((timestamp + 63) % 13 == 0)
        {
            break;
        }
    }
    part2!(timestamp);

}

// https://adventofcode.com/2020/day/13
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    part1(input.clone());
    part2(input.clone());
}
