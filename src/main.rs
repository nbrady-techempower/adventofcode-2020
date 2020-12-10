mod puzzles;
mod utils;

use crate::puzzles::*;
use crate::utils::utils::*;
use crate::utils::int_utils::*;
use crate::utils::str_utils::*;
use crate::utils::vec_utils::*;

// Set this to 0 to run all the days
const DAY: usize = 10;

fn get_input(day: usize) -> String {
    println!("** Day {} **", day);
    read_file(&format!("day{}-input.txt", day))
}

fn main() {
    let puzzles = get_puzzles();
    for p in 0..puzzles.len() {
        let day = p + 1;
        if DAY as usize == day || DAY == 0 {
            puzzles[p](get_input(day));
        }
    }
}
