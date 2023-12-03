use std::cmp::min;
use std::ops::Range;
use regex::Regex;

fn main() {
    let file = std::fs::read_to_string("advent-03/src/input").unwrap();
    let numbers: Vec<i32> = Regex::new(r"\d+")
      .unwrap()
      .find_iter(&file)
      .filter_map(|m| m.as_str().parse().ok())
      .collect();
    let all_numbers_sum: i32 = numbers.iter().sum();
    let lines = get_formatted_input();
    println!("part_1 {}", all_numbers_sum - part_1(&lines));
    println!("part_2 {}", part_2(&lines));

}

fn part_1(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    let regex = Regex::new(r"\d+").unwrap();
    for (index, line) in lines.iter().enumerate() {
        let mut pos = 0;
        while let Some(capture) = regex.captures_iter(&line[pos..]).next() {
            if let Some(number) = capture.get(0) {
                let number_start = pos + number.start();
                let number_end = pos + number.end();

                let index_range = number_start..number_end;

                pos = number_end;

                if check_surroundings(&lines, index, &index_range.clone()) {
                    sum += number.as_str().parse::<i32>().unwrap();
                }
            } else {
                pos = line.len();
            }
        }
    }
    sum
}

fn part_2(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    let regex = Regex::new(r"\*").unwrap();
    for (index, line) in lines.iter().enumerate() {
        let mut pos = 0;
        while let Some(capture) = regex.captures_iter(&line[pos..]).next() {
            if let Some(gear) = capture.get(0) {
                let gear_start = pos + gear.start();
                let gear_end = pos + gear.end();
                let index_range = gear_start..gear_end;
                pos = gear_end;

                if let Ok(number) = find_surrounding_numbers(&lines, index, &index_range.clone()) {
                    sum += number;
                }
            } else {
                pos = line.len();
            }
        }
    }
    sum
}

fn check_surroundings(lines: &Vec<String>, line_index: usize, number_range: &Range<usize>) -> bool {
    for i in line_index.saturating_sub(1)..=min(line_index + 1, lines.len() - 1) {
        let line = &lines[i];
        for captures in Regex::new(r"[^a-zA-Z0-9.]").unwrap().captures_iter(line) {
            let special_char_range = captures.get(0).unwrap().start().saturating_sub(1)..captures.get(0).unwrap().end() + 1;
            if ranges_overlap(&number_range, &special_char_range) {
                return false;
            }
        }
    }

    true
}

fn find_surrounding_numbers(lines: &Vec<String>, line_index: usize, number_range: &Range<usize>) -> Result<i32, i32> {
    let mut number: Vec<i32> = Vec::new();
    for i in line_index.saturating_sub(1)..=min(line_index + 1, lines.len() - 1) {
        let line = &lines[i];
        for captures in Regex::new(r"\d+").unwrap().captures_iter(line) {
            let other_number_range = captures.get(0).unwrap().start().saturating_sub(1)..captures.get(0).unwrap().end() + 1;
            if ranges_overlap(&number_range, &other_number_range) {
                number.push(captures.get(0).unwrap().as_str().parse::<i32>().unwrap());
            }
        }
    }
    return if number.len() >= 2 {
        Ok(number.iter().fold(1, |acc, x| acc * x))
    } else {
        Err(0)
    }
}

fn ranges_overlap(range1: &Range<usize>, range2: &Range<usize>) -> bool {
    range1.start < range2.end && range1.end > range2.start
}
fn get_formatted_input() -> Vec<String> {
    include_str!("input").lines().map(|line| line.to_string()).collect()
}
