use itertools::Itertools;

use crate::util::read_lines;
use std::collections::HashSet;

type Displays = Vec<String>;
type Line = (Displays, Displays);

fn read_displays(input: &str) -> Displays {
  input.split_whitespace().map( | component | component.to_string()).collect()
}

fn read_puzzle(file: &str) -> impl Iterator<Item = Line> {
  read_lines(file, | line | {
    let mut halves = line.split(" | ");
    let (signals_half, value_half) = (halves.next().unwrap(), halves.next().unwrap());
    Some((read_displays(signals_half), read_displays(value_half)))
  }).unwrap()
}

fn part1(input: impl Iterator<Item = Line>) -> usize {
  input.map( | (_, value) | value.iter().filter( | display | {
    let display_len = display.len();
    display_len != 5 && display_len != 6
  }).count()).sum()
}

lazy_static ! {
  static ref DIGITS: Vec<&'static str> = { vec![
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", 
    "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"
  ] };
}

lazy_static ! {
  static ref VALID: HashSet<&'static str> = DIGITS.iter().map(| &digit | digit).collect();
}

fn decode(key: &Vec<char>, cyphertext: &String) -> String {
  cyphertext.chars().map(| c | key[(c as usize) - ('a' as usize)]).sorted().collect()
}

fn solve((signals, value): Line) -> usize {
  let solutions: Vec<Vec<char>> = ('a'..='g').permutations(7).filter(| candidate | {
    signals.iter().all(| display | {
      VALID.contains(decode(candidate, display).as_str())
    })
  }).collect();
  assert_eq!(solutions.len(), 1);
  value.iter().map(| display | { 
    let plaintext = decode(&solutions[0], display);
    DIGITS.iter().position(| &digit | plaintext == digit).unwrap()
  }).fold(0, |acc, digit| acc * 10 + digit)
}

fn part2(input: impl Iterator<Item = Line>) -> usize {
  input.map(solve).sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_read_puzzle() {
    let mut input = read_puzzle("day08.example");
    let (_, value) = input.next().unwrap();
    assert_eq!(value, vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"])
  }

  #[test]
  fn example_day08_part1() {
    let input = read_puzzle("day08.example");
    assert_eq!(26, part1(input));
  }

  #[test]
  fn exec_day08_part1() {
    let input = read_puzzle("day08.txt");
    println!("Day 08, Part 1 – {}", part1(input));
  }

  #[test]
  fn example_day08_part2() {
    let input = read_puzzle("day08.example");
    assert_eq!(61229, part2(input));
  }

  #[test]
  fn exec_day08_part2() {
    let input = read_puzzle("day08.txt");
    println!("Day 08, Part 2 – {}", part2(input));
  }
}