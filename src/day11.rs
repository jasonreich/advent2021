use crate::util::read_lines;

type Puzzle = Vec<Vec<u32>>;

pub fn parse_puzzle(file: &str) -> Puzzle {
  read_lines(file, |line: String| -> Option<Vec<u32>> {
    Some(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
  }).unwrap().collect()
}

fn step(mut input: Puzzle) {
  for x in 0..10 {
    for y in 0..10 {
      let line = input.get_mut(y).unwrap();
      line[x] += 1;
    }
  }
}

#[cfg(test)]
mod test {
  
}