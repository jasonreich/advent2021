use itertools::Itertools;

use crate::util::read_lines;

pub fn parse_puzzle(file: &str) -> impl Iterator<Item = Vec<char>> {
  read_lines(file, |line: String| -> Option<Vec<char>> {
    Some(line.chars().collect())
  }).unwrap()
}

pub fn score(line: Vec<char>) -> u64 {
  let mut stack: Vec<char> = Vec::new();
  for char in line {
    match char {
      '[' => stack.push(']'),
      '(' => stack.push(')'),
      '{' => stack.push('}'),
      '<' => stack.push('>'),
      other => {
        if other != stack.pop().unwrap() {
          return match other {
              ')' => 3,
              ']' => 57,
              '}' => 1197,
              '>' => 25137,
              _ => unimplemented!()
          }
        }
      },
    }
  }
  0
}

pub fn part1(input: impl Iterator<Item = Vec<char>>) -> u64 {
  input.map(score).sum()
}

pub fn fix(line: Vec<char>) -> u64 {
  let mut stack: Vec<char> = Vec::new();
  for char in line {
    match char {
      '[' => stack.push(']'),
      '(' => stack.push(')'),
      '{' => stack.push('}'),
      '<' => stack.push('>'),
      other => {
        if other != stack.pop().unwrap() {
          return 0
        }
      },
    }
  }
  stack.iter().rev().fold(0, |acc, char| {
    acc * 5 + match char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unimplemented!(),
    }
  })
}

pub fn part2(input: impl Iterator<Item = Vec<char>>) -> u64 {
  let scores: Vec<u64> = input.map(fix).filter(|&n| n != 0).sorted().collect();
  scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_puzzle() {
    let mut input = parse_puzzle("day10.example");
    assert_eq!(&'>', input.next().unwrap().last().unwrap());
  }

  #[test]
  fn example_day10_part1() {
    let input = parse_puzzle("day10.example");
    assert_eq!(26397, part1(input));
  }

  #[test]
  fn exec_day10_part1() {
    let input = parse_puzzle("day10.txt");
    println!("Day 10 Part 1 - {}", part1(input));
  }

  #[test]
  fn example_day10_fix() {
    let input = parse_puzzle("day10.example");
    let fixes: Vec<u64> = input.map(fix).filter(|&n| n > 0).collect();
    assert_eq!(vec![288957, 5566, 1480781, 995444, 294], fixes);
  }
  
  #[test]
  fn example_day10_part2() {
    let input = parse_puzzle("day10.example");
    assert_eq!(288957, part2(input));
  }

  #[test]
  fn exec_day10_part2() {
    let input = parse_puzzle("day10.txt");
    println!("Day 10 Part 2 - {}", part2(input));
  }
  
}