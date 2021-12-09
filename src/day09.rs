use crate::util::read_lines;

pub fn parse_puzzle(file: &str) -> Vec<Vec<u32>> {
  read_lines(file, |line: String| -> Option<Vec<u32>> {
    Some(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
  }).unwrap().collect()
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
}