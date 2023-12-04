use std::cmp::{max, min};
use std::collections::VecDeque;

fn main() {
  let input = get_formatted_input();
  println!("{}", round_1(&input.clone()));
  println!("{}", round_2(&input.clone()));
}

fn round_1(input: &Vec<Vec<i32>>) -> i32 {
  let mut sum = 0;
  input.iter().for_each(|x| {
    if x.len() == 0 {
      return;
    }
    let mut res = 1;
    let mut first_iteration = true;
    x.iter().for_each(|y| {
      if first_iteration {
        first_iteration = false;
      } else {
        res *= 2;
      }
    });
    sum += res;
  });

  sum
}

fn round_2(input: &Vec<Vec<i32>>) -> i32 {
  let asd = input.clone();
  let mut vecs: VecDeque<VecDeque<&Vec<i32>>> = VecDeque::new();
  let mut i = 0;

  while i < input.len() {
    if let Some(inner_vec) = input.get(i) {
      vecs.push_back(VecDeque::from(vec![inner_vec]));
      i += 1;
    } else {
      break;
    }
  }

  let mut index = 0;
  let len = vecs.len();

  while index < len {
    let mut vec = &vecs[index].clone(); // [[83, 86, 17, 48]]
    for a in 0..vec.len() {
      for b in 0..vec[a].len() {
        if let Some(next_vec) = asd.get(index + b + 1) {
          vecs[index + b + 1].push_back(next_vec);
        }
      }
    }
    index += 1;
  }

  let mut res: usize = vecs.iter().map(|x| x.len()).collect::<Vec<usize>>().iter().sum();
  res as i32
}

fn get_formatted_input() -> Vec<Vec<i32>> {
  include_str!("input").lines().map(|line| {
    let first_split: Vec<&str> = line.rsplit_terminator(": ").collect::<Vec<&str>>();
    let second_split: Vec<&str> = first_split[0].rsplit_terminator(" | ").collect::<Vec<&str>>();
    let my_numbers: Vec<i32> = second_split[0].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let winning_numbers: Vec<i32> = second_split[1].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    (my_numbers, winning_numbers)
  }).into_iter()
    .map(|(my_numbers, winning_numbers)| {
      my_numbers
        .into_iter()
        .filter(move |&num| winning_numbers.contains(&num))
        .collect()
    })
    .collect()
}


