use std::collections::HashMap;
use regex::Regex;

fn main() {
    let max_red_cubes: i32 = 12;
    let max_green_cubes: i32 = 13;
    let max_blue_cubes: i32 = 14;
    let mut possible_game_id_sum: i32 = 0;
    let mut set_power: i32 = 0;

    include_str!("input").lines().enumerate().for_each(|(index, line)| {
        let round: i32 = index as i32 + 1;
        let mut round_finished: bool = false;

        let regex = Regex::new(r"(\d+)\s+(red|green|blue)").unwrap();

        let mut color_values: HashMap<&str, Vec<i32>> = HashMap::new();

        for capture in regex.captures_iter(line) {
            let number = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let color = capture.get(2).unwrap().as_str();

            color_values.entry(color).or_insert(vec![]).push(number);
        }

        color_values.iter().for_each(|(color, values)| {
            values.iter().for_each(|value| {
                if color == &"red" && value > &max_red_cubes && !round_finished {
                    round_finished = true;
                } else if color == &"green" && value > &max_green_cubes && !round_finished {
                    round_finished = true;
                } else if color == &"blue" && value > &max_blue_cubes && !round_finished {
                    round_finished = true;
                }
            });

        });

        if !round_finished {
            possible_game_id_sum += round;
        }
        let max_red = color_values.get("red").map_or(0, |values| *values.iter().max().unwrap_or(&0));
        let max_green = color_values.get("green").map_or(0, |values| *values.iter().max().unwrap_or(&0));
        let max_blue = color_values.get("blue").map_or(0, |values| *values.iter().max().unwrap_or(&0));
        set_power += max_red * max_green * max_blue;
        println!("{}: {} {} {} = {}", round, max_red, max_green, max_blue, set_power);
    });

    println!("{}", possible_game_id_sum);
    println!("{}", set_power);
}
