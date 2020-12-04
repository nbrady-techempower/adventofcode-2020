pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub fn get_puzzles() -> Vec<fn(String)> {
    vec![
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve
    ]
}