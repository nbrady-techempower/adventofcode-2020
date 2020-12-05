use crate::*;

fn lets_go(input: &Vec<&str>) {
    let mut seat_ids = vec![];
    let mut max_seat_id = 0;
    for i in input {
        let mut row = 127;
        let mut cur_row = 64;

        let mut col = 7;
        let mut cur_col = 4;

        for c in i.chars().into_iter() {
            row -= i32!(c == 'F') * cur_row;
            col -= i32!(c == 'L') * cur_col;
            cur_row = (cur_row / 2) as i32;
            if (c == 'L') || (c == 'R') {
                cur_col = (cur_col / 2) as i32;
            }
        }
        let seat_id = row * 8 + col;
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
        seat_ids.push(seat_id);
    }

    part1!(max_seat_id);

    seat_ids.sort();
    for (idx, s) in seat_ids.iter().enumerate() {
        if *s as usize != idx + 12 {
            part2!(s-1);
            break;
        }
    }
}

// https://adventofcode.com/2020/day/5
pub fn solve(input: String) {
    let input: Vec<&str> = input.split("\n").collect();
    lets_go(&input);
}
