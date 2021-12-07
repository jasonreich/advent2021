type Crabs = Vec<i32>;

pub fn part1(input: &Crabs) -> i32 {
  let lowest = input.iter().min().unwrap();
  let highest = input.iter().max().unwrap();

  (*lowest..=*highest).map(|target| 
    input.iter().map(|source| (target - source).abs()).sum()
  ).min().unwrap()
}

pub fn part2(input: &Crabs) -> i32 {
  let lowest = input.iter().min().unwrap();
  let highest = input.iter().max().unwrap();

  (*lowest..=*highest).map(|target| 
    input.iter().map(|source| {
      let distance = (target - source).abs();
      distance * (distance + 1) / 2
    }).sum()
  ).min().unwrap()
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::util::read_line;

  #[test]
  fn example_day07_part1() {
    let example_input: Crabs = vec![16,1,2,0,4,2,7,1,2,14];
    assert_eq!(37, part1(&example_input));
  }

  #[test]
  fn exec_day07_part1() {
    let input: Crabs = read_line("day07.txt");
    println!("Day 07 Part 1: {}", part1(&input));
  }

  #[test]
  fn example_day07_part2() {
    let example_input: Crabs = vec![16,1,2,0,4,2,7,1,2,14];
    assert_eq!(168, part2(&example_input));
  }

  #[test]
  fn exec_day07_part2() {
    let input: Crabs = read_line("day07.txt");
    println!("Day 07 Part 2: {}", part2(&input));
  }
}