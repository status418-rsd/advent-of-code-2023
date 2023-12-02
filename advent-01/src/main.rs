use regex::Regex;
use std::collections::HashMap;

fn main() {
    let mut part_one_sum = part_one();
    let mut part_two_sum = part_two();
    println!("{}", part_one_sum);
    println!("{}", part_two_sum);
    // fancy_part_one();
}

fn part_one() -> i32 {
    let mut sum = 0;
    let mut peekable = include_str!("input").lines().peekable();
    while let Some(line) = peekable.next() {
        if line.is_empty() {
            continue;
        }
        let number_pattern = Regex::new(r"\d").unwrap();
        // Get the first number in the line
        let first_number = get_number(line, &number_pattern);
        // Get the last number in the line
        let last_number = get_number(line.chars().rev().collect::<String>().as_mut_str(), &number_pattern);
        let combined: String = first_number.to_string() + &last_number.to_string();

        sum += combined.parse::<i32>().unwrap();
    }
    // println!("{}", sum);
    sum
}

// fn fancy_part_one() -> i32 {
//     let mut peekable = include_str!("input").lines().peekable();
//     let mut a = peekable.map(|line| {
//         line.chars()
//           .enumerate()
//           .filter_map(|(index, c)| c.to_digit(10).map(|d| (index, d as usize)))
//     });
//     println!("{:?}", a);
//     32
// }


fn part_two() -> i32 {
    let mut sum = 0;
    let mut i = 1;
    let mut peekable = include_str!("input").lines().peekable();
    while let Some(line) = peekable.next() {
        if line.is_empty() {
            continue;
        }
        // println!("\n {}: {}", i, line);
        let word_numbers: Vec<&str> = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].to_vec();


        let re_number = Regex::new(r#"(?P<number>\d)"#).unwrap();
        let re_word_number = Regex::new(r#"(?P<word_number>one|two|three|four|five|six|seven|eight|nine)"#).unwrap();

        let mut pos = 0;
        let mut line_numbers = Vec::new();
        let earliest_number_pos = re_number.find(line).map(|number| number.start());
        let earliest_word_pos = re_word_number.find(line).map(|word_number| word_number.start());

        while pos < line.len() {
            if earliest_number_pos < earliest_word_pos {
                if let Some(number) = re_number.find(&line[pos..]) {
                    line_numbers.push(number.as_str().to_string());
                    pos += number.end();
                } else if let Some(word_number) = re_word_number.find(&line[pos..]) {
                    let converted_word_number = convert_word_numbers(word_number.as_str()).to_string();
                    line_numbers.push(converted_word_number.as_str().parse().unwrap());
                    pos += word_number.end() - 1;
                } else {
                    pos += 1;
                }
            } else {
                if let Some(word_number) = re_word_number.find(&line[pos..]) {
                    let converted_word_number = convert_word_numbers(word_number.as_str()).to_string();
                    line_numbers.push(converted_word_number.as_str().parse().unwrap());
                    pos += word_number.end() - 1;
                } else if let Some(number) = re_number.find(&line[pos..]) {
                    line_numbers.push(number.as_str().to_string());
                    pos += number.end();
                } else {
                    pos += 1;
                }
            }
        }
        // let numbers = extract_numbers(&converted_line);
        // println!("line numbers {:?}", line_numbers);

        // Get the first number in the line
        if let Some(first) = line_numbers.first() {
            if let Some(last) = line_numbers.last() {
                let concatenated_numbers = format!("{}{}", first, last);
                // println!("Concatenated Numbers: {}", concatenated_numbers);
                sum += concatenated_numbers.parse::<i32>().unwrap();
            }
        }
        // let combined: String = line_numbers.first().to_string() + &line_numbers.last().to_string();
        // println!("{}", combined);
        i += 1;
    }
    sum
}

fn get_number(line: &str, pattern: &Regex) -> i32 {
    if line.is_empty() {
        return 0;
    }
    let number = pattern
        .find(line)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
    number
}

fn convert_word_numbers(word_number: &str) -> String {
    let mut word_to_number = HashMap::new();
    word_to_number.insert("one", "1");
    word_to_number.insert("two", "2");
    word_to_number.insert("three", "3");
    word_to_number.insert("four", "4");
    word_to_number.insert("five", "5");
    word_to_number.insert("six", "6");
    word_to_number.insert("seven", "7");
    word_to_number.insert("eight", "8");
    word_to_number.insert("nine", "9");

    word_to_number.get(word_number).unwrap().to_string()

}
