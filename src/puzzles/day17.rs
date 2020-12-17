use crate::*;
use std::collections::HashSet;

fn num_active(grid: &Grid) -> i32 {
    let mut to_ret = 0;
    for dir in [DIR::N, DIR::S, DIR::E, DIR::W, DIR::NW, DIR::NE, DIR::SW, DIR::SE].iter() {
        if let Some(c) = grid.look(dir) {
            to_ret += i32!(c == '#');
        }
    }
    to_ret
}

// It was a mistake to reuse my grid data structure from day 11
fn part1(input: Vec<String>) {
    let mut iterations = 0;
    let mut all: Vec<Grid> = vec!();
    all.push(Grid::new_padded(input.clone()));
    while iterations < 6 {
        iterations += 1;
        all.push(Grid::new_empty(all[0].num_rows, all[0].num_cols));
        let mut new_grids: Vec<Grid> = vec!();
        for (grid_idx, grid) in all.iter().enumerate() {
            let mut new_grid: Vec<String> = vec!();
            for y in 0..grid.num_rows {
                let mut s: String = "".to_string();
                for x in 0..grid.num_cols {
                    let mut g = grid.clone();
                    if let Some(ch) = g.move_to(Point { x: x as i32, y: y as i32 }) {
                        let mut num = num_active(&g);
                        if grid_idx == 0 {
                            let mut front = all[grid_idx + 1].clone();
                            if let Some(c) = &front.move_to(Point { x: x as i32, y: y as i32 }) {
                                num += 2 * i32!(*c == '#');
                            }
                            num += 2 * num_active(&front);
                        } else if grid_idx == all.len() - 1 {
                            let mut back = all[grid_idx - 1].clone();
                            if let Some(c) = &back.move_to(Point { x: x as i32, y: y as i32 }) {
                                num += i32!(*c == '#');
                            }
                            num += num_active(&back);
                        } else {
                            let mut back = all[grid_idx - 1].clone();
                            let mut front = all[grid_idx + 1].clone();
                            if let Some(c) = &back.move_to(Point { x: x as i32, y: y as i32 }) {
                                num += i32!(*c == '#');
                            }
                            num += num_active(&back);
                            if let Some(c) = &front.move_to(Point { x: x as i32, y: y as i32 }) {
                                num += i32!(*c == '#');
                            }
                            num += num_active(&front);
                        }
                        match num {
                            2 if ch == '#' => s = string!(s, '#'),
                            3 => s = string!(s, '#'),
                            _ => s = string!(s, '.')
                        }
                    }
                }
                new_grid.push(s);
            }
            let new_grid = Grid::new_padded(new_grid.clone());
            new_grids.push(new_grid);
        }
        all = new_grids;
    }

    let mut count = 0;
    for (i, grid) in all.iter().enumerate() {
        if i == 0 {
            count += grid.count_char('#');
        } else {
            count += 2 * grid.count_char('#');
        }
    }
    part1!(count);
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Space(i64, i64, i64, i64);

trait Neighbors
    where Self: std::marker::Sized + Eq + std::hash::Hash + Copy
{
    fn get_neighbors(&self) -> Vec<Self>;
}

impl Neighbors for Space {
    fn get_neighbors(&self) -> Vec<Self> {
        let mut to_ret = vec![];
        let curr = (self.0, self.1, self.2, self.3);
        let (x, y, z, a) = curr;
        for nx in x-1..=x+1 {
            for ny in y-1..=y+1 {
                for nz in z- 1..=z+1 {
                    for na in a-1..=a+1 {
                        if (nx, ny, nz, na) != curr {
                            to_ret.push(Space(nx, ny, nz, na));
                        }
                    }
                }
            }
        }
        to_ret
    }
}

fn get_inactives<A: Neighbors>(actives: &HashSet<A>) -> HashSet<A> {
    let mut inactives = HashSet::new();
    for &active in actives.iter() {
        for &adj in active.get_neighbors().iter() {
            if !actives.contains(&adj) {
                inactives.insert(adj);
            }
        }
    }

    inactives
}

fn step<A: Neighbors>(actives: HashSet<A>) -> HashSet<A> {
    let inactives = get_inactives(&actives);
    let mut next = HashSet::new();

    for &active in actives.iter() {
        if (2..=3).contains(
            &active
                .get_neighbors()
                .iter()
                .filter(|&xyz| actives.contains(xyz))
                .count(),
        ) {
            next.insert(active);
        }
    }

    for &inactive in inactives.iter() {
        if 3 == inactive
            .get_neighbors()
            .iter()
            .filter(|&xyz| actives.contains(xyz))
            .count()
        {
            next.insert(inactive);
        }
    }

    next
}

fn sim<A: Neighbors>(mut actives: HashSet<A>) -> usize {
    for _ in 0..6 {
        actives = step(actives);
    }
    actives.iter().count()
}

// Disclaimer: This is not my full solution.
// I wrote a 400 line behemoth based on my Grid data structure used in part 1. It got me the
// correct solution but it was absolutely terrible. I took this from a rust solution in the
// megathread to alter it to help me understand generics. Though I understand this solution now,
// I don't think I could recreate it without looking at this.
fn part2(input: Vec<String>) {
    let mut pairs = HashSet::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                pairs.insert((x as i64, y as i64));
            }
        }
    }
    part2!(sim(pairs.iter().map(|&(x, y)| Space(x, y, 0, 0)).collect()));
}

// https://adventofcode.com/2020/day/17
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    part1(input.clone());
    part2(input.clone());
}
