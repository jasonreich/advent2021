use std::collections::HashSet;

pub fn parse_input(input: &mut impl Iterator<Item = String>) -> (Calls, Vec<Board>) {
  let calls = parse_calls(input.next().unwrap());
  input.next();

  let mut boards: Vec<Board> = Vec::new();

  while let Some(board) = parse_board(&mut *input) {
    boards.push(build_board(board));
  }

  (calls, boards)
}

type Calls = Vec<u32>;

fn parse_calls(input: String) -> Calls {
  input.split(",").map(|part| part.parse().unwrap()).collect()
}

fn parse_board(mut input: impl Iterator<Item = String>) -> Option<Vec<Vec<u32>>> {
  let mut board: Vec<Vec<u32>> = Vec::new();
  while let Some(line) = input.next() {
    if line.is_empty() {
      break;
    } else {
      board.push(
        line
          .split_whitespace()
          .map(|column| column.parse().unwrap())
          .collect(),
      );
    }
  }
  
  if board.is_empty() {
    None
  } else {
    Some(board)
  }
}

type Board = Vec<HashSet<u32>>;

fn build_board(input: Vec<Vec<u32>>) -> Board {
  let rows: Vec<HashSet<u32>> = input
    .iter()
    .map(|row| row.iter().copied().collect())
    .collect();

  let size = input[0].len();
  let columns: Vec<HashSet<u32>> = (0..size)
    .map(|column_index| input.iter().map(|row| row[column_index]).collect())
    .collect();

  [rows, columns].concat()
}

pub fn part1(calls: Calls, mut boards: Vec<Board>) -> Option<u32> {
  for call in calls {
    for board in boards.as_mut_slice() {
      let mut line_signal = false;
      board.iter_mut().for_each(| strip | { 
        strip.remove(&call);
        if strip.is_empty() {
          line_signal = true;
        }
      });
      if line_signal {
        let uniques: HashSet<u32> = board.iter().flatten().copied().collect();
        let sum: u32 = uniques.iter().sum();
        return Some(sum * call);
      }
    }
  }
  return None;
}

pub fn part2(calls: Calls, mut boards: Vec<Board>) -> Option<u32> {
  let mut last_winner: Option<u32> = None;
  for call in calls {
    for board in boards.as_mut_slice() {
      let mut line_signal = false;
      if board.iter().all(| strip | !strip.is_empty()) {
        board.iter_mut().for_each(| strip | { 
          strip.remove(&call);
          if strip.is_empty() {
            line_signal = true;
          }
        });
      }
      if line_signal {
        let uniques: HashSet<u32> = board.iter().flatten().copied().collect();
        let sum: u32 = uniques.iter().sum();
        last_winner = Some(sum * call);
      }
    }
  }
  return last_winner;
}

#[cfg(test)]
mod test {
  use crate::util::read_file;
  use super::*;

  #[test]
  fn test_parse_calls() {
    assert_eq!(vec![1, 2, 3, 4], parse_calls("1,2,3,4".to_string()))
  }

  #[test]
  fn test_parse_board() {
    let mut input = "
    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    "
    .lines()
    .map(|line| line.to_string());

    input.next();
    assert_eq!(14, parse_board(input).unwrap()[2][2])
  }

  #[test]
  fn test_build_board() {
    let input = vec![
      vec![1, 2, 3],
      vec![4, 5, 6],
      vec![7, 8, 9],
    ];

    build_board(input);
  }

  #[test]
  fn example_day04_part1() {
    let mut lines = read_file("day04.example").unwrap();

    let (calls, boards) = parse_input(&mut lines);

    let result = part1(calls, boards);

    assert_eq!(result, Some(4512));
  }

  #[test]
  fn exec_day04_part1() {
    let mut lines = read_file("day04.txt").unwrap();

    let (calls, boards) = parse_input(&mut lines);

    let result = part1(calls, boards).unwrap();

    println!("Day 4 Part 1: {}", result);
  }
  
  #[test]
  fn example_day04_part2() {
    let mut lines = read_file("day04.example").unwrap();

    let (calls, boards) = parse_input(&mut lines);

    let result = part2(calls, boards);

    assert_eq!(result, Some(1924));
  }

  #[test]
  fn exec_day04_part2() {
    let mut lines = read_file("day04.txt").unwrap();

    let (calls, boards) = parse_input(&mut lines);

    let result = part2(calls, boards).unwrap();

    println!("Day 4 Part 2: {}", result);
  }
}