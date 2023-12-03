use std::collections::HashMap;

use regex::Regex;

fn main() {
  let color_values_per_game: Vec<HashMap<String, Vec<i32>>> = get_formatted_input();
  let color_limits: [(&str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

  println!("{}", round_1(&color_values_per_game, &color_limits));
  println!("{}", round_2(&color_values_per_game, &color_limits));
}

fn round_1(color_values_per_game: &Vec<HashMap<String, Vec<i32>>>, color_limits: &[(&str, i32); 3]) -> i32 {
  color_values_per_game.iter().enumerate().fold(0, |acc, (index, color_values_map)| {
    let round: i32 = index as i32 + 1;
    if !color_limits
      .iter()
      .any(|&(color, max_cubes)| {
        color_values_map
          .get(color)
          .map_or(false, |values| values.iter().any(|value| value > &max_cubes))
      }) {
      acc + round
    } else {
      acc
    }
  })
}

fn round_2(color_values_per_game: &Vec<HashMap<String, Vec<i32>>>, color_limits: &[(&str, i32); 3]) -> i32 {
  color_values_per_game.iter().fold(0, |acc, color_values_map| {
    acc + color_limits
      .iter()
      .map(|&(color, _)| calculate_max_color_value(&color_values_map, color))
      .fold(1, |acc, x| acc * x)
  })
}

fn get_formatted_input() -> Vec<HashMap<String, Vec<i32>>> {
  let regex = Regex::new(r"(\d+)\s+(red|green|blue)").unwrap();
  include_str!("input").lines().map(|line| {
    regex.captures_iter(line).fold(HashMap::new(), |mut acc, capture| {
      acc.entry(capture.get(2).unwrap().as_str().to_string())
        .and_modify(|v| v.push(capture.get(1).unwrap().as_str().parse().unwrap()))
        .or_insert_with(|| vec![capture.get(1).unwrap().as_str().parse().unwrap()]);
      acc
    })
  }).collect()
}

fn calculate_max_color_value(color_values: &HashMap<String, Vec<i32>>, color: &str) -> i32 {
  color_values.get(color).map_or(0, |values| *values.iter().max().unwrap_or(&0))
}
