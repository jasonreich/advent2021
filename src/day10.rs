use core::panic;

use crate::util::read_lines;

pub fn parse_puzzle(file: &str) -> impl Iterator<Item = Vec<char>> {
  read_lines(file, |line: String| -> Option<Vec<char>> {
    Some(line.chars().collect())
  }).unwrap()
}

pub fn score(line: Vec<char>) -> u32 {
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
              _ => panic!(),
          }
        }
      },
    }
  }
  0
  // unimplemented!()
}

fn part1(input: impl Iterator<Item = Vec<char>>) -> u32 {
  input.map(score).sum()
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
  
  // #[test]
  // fn example_day10_part2() {
  //   let input = parse_puzzle("day10.example");
  //   assert_eq!(1134, part2(input));
  // }

  // #[test]
  // fn exec_day10_part2() {
  //   let input = parse_puzzle("day10.txt");
  //   println!("Day 10 Part 2 - {}", part2(input));
  // }
  
}