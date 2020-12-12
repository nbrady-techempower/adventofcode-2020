use crate::*;

fn lets_go(part: u8, input: Vec<String>) {

    let mut facing = "E";
    let mut x = 0;
    let mut y = 0;

    for i in input {
        let d = i[0..1].as_ref();
        let mut num = i32!(i[1..].to_string());

        match d {
            "R" => {
                while num > 0 {
                    match facing {
                        "E" => facing = "S",
                        "W" => facing = "N",
                        "S" => facing = "W",
                        "N" => facing = "E",
                        _ => ()
                    }
                    num -= 90;
                }
            },
            "L" => {
                while num > 0 {
                    match facing {
                        "E" => facing = "N",
                        "W" => facing = "S",
                        "S" => facing = "E",
                        "N" => facing = "W",
                        _ => ()
                    }
                    num -= 90;
                }
            },
            "S" => y -= num,
            "N" => y += num,
            "E" => x += num,
            "W" => x -= num,
            "F" => {
                match facing {
                    "S" => y -= num,
                    "N" => y += num,
                    "E" => x += num,
                    "W" => x -= num,
                    _ => ()
                }
            },
            _ => ()
        }

    }
    println!(" Part {}: {}", part, x.abs() + y.abs());
}

fn lets_go2(part: u8, input: Vec<String>) {

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
    println!(" Part {}: {}", part, x.abs() + y.abs());
}


// https://adventofcode.com/2020/day/12
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    lets_go(1, input.clone());
    lets_go2(2, input.clone());
}
