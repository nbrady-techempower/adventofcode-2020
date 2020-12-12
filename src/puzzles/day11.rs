use crate::*;

fn num_seats(grid: &Grid) -> i32 {
    let mut to_ret = 0;
    for dir in [DIR::N, DIR::S, DIR::E, DIR::W, DIR::NW, DIR::NE, DIR::SW, DIR::SE].iter() {
        if let Some(c) = grid.look(dir) {
            to_ret += i32!(c == '#');
        }
    }
    to_ret
}

fn num_seats2(grid: &Grid) -> i32 {
    let mut grid = grid.clone();
    let mut to_ret = 0;
    let pos = grid.pos.clone();

    for dir in [DIR::N, DIR::S, DIR::E, DIR::W, DIR::NW, DIR::NE, DIR::SW, DIR::SE].iter() {
        while let Some(c) = grid.moves(dir) {
            match c {
                '#' => {
                    to_ret += 1;
                    break;
                }
                'L' => break,
                _ => ()
            }
        }
        grid.pos = pos;
    }

    to_ret
}

fn lets_go(part: u8, input: Vec<String>, seat_rule: i32, func: fn(&Grid) -> i32) {
    let mut old = Grid::new(vec!());
    let mut new = Grid::new(input.clone());

    while old != new {
        old = new.clone();
        for p in old.clone() {
            old.move_to(p);
            let c = old.get_point(p).unwrap();
            let num_seats = func(&old);
            let mut new_char= c;
            match c {
                'L' if num_seats == 0 => new_char = '#',
                '#' if num_seats >= seat_rule => new_char = 'L',
                _ => ()
            }
            new.set_point(p, new_char);
        }
    }
    println!(" Part {}: {}", part, old.count_char('#'));
}

// https://adventofcode.com/2020/day/11
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    lets_go(1, input.clone(), 4, num_seats);
    lets_go(2, input.clone(), 5, num_seats2);
}
