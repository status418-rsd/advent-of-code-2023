use std::collections::HashMap;
use regex::Regex;

fn main() {
    let color_limits = [("red", 12), ("green", 13), ("blue", 14)];
    let regex = Regex::new(r"(\d+)\s+(red|green|blue)").unwrap();
    let mut round_1: i32 = 0;
    let mut round_2: i32 = 0;

    include_str!("input").lines().enumerate().for_each(|(index, line)| {
        let round: i32 = index as i32 + 1;

        let mut color_values: HashMap<&str, Vec<i32>> = regex
          .captures_iter(line)
          .fold(HashMap::new(), |mut acc, capture| {
              acc.entry(capture.get(2).unwrap().as_str())
                .and_modify(|v| v.push(capture.get(1).unwrap().as_str().parse().unwrap()))
                .or_insert_with(|| vec![capture.get(1).unwrap().as_str().parse().unwrap()]);
              acc
          });

        if !color_limits
          .iter()
          .any(|&(color, max_cubes)| {
              color_values
                .get(color)
                .map_or(false, |values| values.iter().any(|value| value > &max_cubes))
          }) {
            round_1 += round;
        }

        round_2 += color_limits
          .iter()
          .map(|&(color, _)| calculate_max_color_value(&color_values, color))
          .fold(1, |acc, x| acc * x);
    });

    println!("{}", round_1);
    println!("{}", round_2);
}

fn calculate_max_color_value(color_values: &HashMap<&str, Vec<i32>>, color: &str) -> i32 {
    color_values.get(color).map_or(0, |values| *values.iter().max().unwrap_or(&0))
}
