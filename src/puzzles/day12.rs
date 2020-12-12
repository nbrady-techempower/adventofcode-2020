use crate::*;

fn part1(input: Vec<String>) {
    let mut facing = ["E", "S", "W", "N"];
    let mut x = 0;
    let mut y = 0;

    for i in input {
        let mut d = i[0..1].as_ref();
        let num = i32!(i[1..].to_string());

        match d {
            "R" => facing.rotate_left((num / 90) as usize),
            "L" => facing.rotate_right((num / 90) as usize),
            _ => {
                if d == "F" {
                    d = facing[0];
                }
                match d {
                    "S" => y -= num,
                    "N" => y += num,
                    "E" => x += num,
                    "W" => x -= num,
                    _ => ()
                }
            }
        }

    }
    part1!(x.abs() + y.abs());
}

fn part2(input: Vec<String>) {
    let mut waypoint = vec![1, 10, 0, 0];
    let mut x = 0;
    let mut y = 0;

    for i in input {
        let d = i[0..1].as_ref();
        let num = i32!(i[1..].to_string());

        match d {
            "N" => waypoint[0] += num,
            "E" => waypoint[1] += num,
            "S" => waypoint[2] += num,
            "W" => waypoint[3] += num,
            "R" => waypoint.rotate_right((num / 90) as usize),
            "L" => waypoint.rotate_left((num / 90) as usize),
            "F" => {
                y -= num * waypoint[0];
                y += num * waypoint[2];
                x -= num * waypoint[3];
                x += num * waypoint[1];
            },
            _ => ()
        }

    }
    part2!(x.abs() + y.abs());
}


// https://adventofcode.com/2020/day/12
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    part1(input.clone());
    part2( input.clone());
}
