use crate::util::read_lines;

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
    assert_eq!(26, part1(input))
  }

  #[test]
  fn example_day08_part2() {
    let input = read_puzzle("day08.txt");
    println!("Day 08, Part 1 – {}", part1(input))
  }
}