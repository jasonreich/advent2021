use crate::util::read_lines;

type Puzzle = Vec<Vec<u32>>;

pub fn parse_puzzle(file: &str) -> Puzzle {
  read_lines(file, |line: String| -> Option<Vec<u32>> {
    Some(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
  }).unwrap().collect()
}

fn get(input: &Puzzle, x: Option<usize>, y: Option<usize>) -> Option<u32> {
  let x = x?;
  let y = y?;
  let line = input.get(y)?;
  let cell = line.get(x)?;
  Some(*cell)
}

fn add1(input: Option<usize>) -> Option<usize> {
  input.and_then(| n | n.checked_add(1))
}

fn minus1(input: Option<usize>) -> Option<usize> {
  input.and_then(| n | n.checked_sub(1))
}

pub fn part1(input: Puzzle) -> u32 {
  let mut risk  = 0;
  for y in (0..input.len()).map(Some) {
    for x in (0..input[0].len()).map(Some) {
      let focus = get(&input, x, y).unwrap();
      let l = get(&input, minus1(x), y).unwrap_or(u32::MAX);
      let r = get(&input, add1(x), y).unwrap_or(u32::MAX);
      let u = get(&input, x, minus1(y)).unwrap_or(u32::MAX);
      let d = get(&input, x, add1(y)).unwrap_or(u32::MAX);
      if focus < l && focus < r && focus < u && focus < d {
        risk += focus + 1;
      }
    }
  }
  risk
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_puzzle() {
    let input = parse_puzzle("day09.example");
    assert_eq!(5, input.len());
    assert_eq!(10, input[0].len());
  }

  #[test]
  fn example_day09_part1() {
    let input = parse_puzzle("day09.example");
    assert_eq!(15, part1(input));
  }

  #[test]
  fn exec_day09_part1() {
    let input = parse_puzzle("day09.txt");
    println!("Day 09 Part 1 - {}", part1(input));
  }
}