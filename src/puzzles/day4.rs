use crate::*;
use crate::utils::*;

fn found_min_max_year(input: &str, pattern: &str, min: i64, max: i64) -> bool {
    if let Some(idx) = input.find(pattern) {
        let num = input[idx+4..idx+8].parse::<i64>().unwrap_or(0);
        return (num >= min) && (num <= max);
    }
    false
}

fn part1(input: &Vec<&str>) {
    let mut valid = 0;
    let mut cur_str = "".to_string();
    for i in input.iter() {
        if i.len() == 0 {
            if (&cur_str[..]).contains_all(vec!["ecl:", "eyr:", "pid:", "hcl:", "byr:", "iyr:", "hgt:"]) {
                valid += 1;
            }
            cur_str = "".to_string();
        } else {
            cur_str = " ".to_string() + &cur_str + i;
        }
    }

    part1!(valid);
}

fn part2(input: &Vec<&str>) {
    let mut valid = 0;
    let mut fields = 7;
    for i in input.iter() {
        if i.len() == 0 {
            if fields == 0 {
                valid += 1;
            }
            fields = 7;
            continue;
        }
        fields -= i.contains_any(vec!["ecl:amb", "ecl:blu", "ecl:brn", "ecl:gry", "ecl:grn", "ecl:hzl", "ecl:oth"]) as i32;
        fields -= i.contains("hcl:#") as i32;

        fields -= found_min_max_year(i, "byr:", 1920, 2002) as i32;
        fields -= found_min_max_year(i, "eyr:", 2020, 2030) as i32;
        fields -= found_min_max_year(i, "iyr:", 2010, 2020) as i32;

        if i.contains("pid:") {
            let idx = i.find("pid:").unwrap()+4;
            if i.len() >= idx+9 {
                let num1 = i[idx..idx+9].parse::<i64>().unwrap_or(0);
                let mut num2 = 0;
                if i.len() >= idx+10 {
                    num2 = i[idx..idx+10].parse::<i64>().unwrap_or(0);
                }
                if num1 > 0 && num2 == 0 {
                    fields -= 1;
                }
            }

        }
        if i.contains_all(vec!["hgt:", "cm"]) {
            let idx = i.find("hgt:").unwrap()+4;
            let num = i[idx..idx+3].parse::<i64>().unwrap_or(0);
            fields -= ((num >= 150) && (num <= 193)) as i32;
        }
        if i.contains_all(vec!["hgt:", "in"]) {
            let idx = i.find("hgt:").unwrap()+4;
            let num = i[idx..idx+2].parse::<i64>().unwrap_or(0);
            fields -= ((num >= 59) && (num <= 76)) as i32;
        }
    }

    part2!(valid);
}

// https://adventofcode.com/2020/day/4
pub fn solve(input: String) {
    let input: Vec<&str> = input.split("\n").collect();
    part1(&input);
    part2(&input);
}
