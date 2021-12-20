use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Number {
    values: Vec<u8>,
    depth: Vec<u8>,
}

impl Number {
    fn parse(input: &str) -> Number {
        let mut values = vec![];
        let mut depths = vec![];
        let mut building = false;
        let mut acc = 0;
        let mut depth = 0;
        for c in input.chars() {
            match c {
                '[' => {
                    depth += 1;
                }
                ']' => {
                    if building {
                        values.push(acc);
                        depths.push(depth - 1);
                    }
                    building = false;
                    acc = 0;
                    depth -= 1;
                }
                ',' => {
                    if building {
                        values.push(acc);
                        depths.push(depth - 1);
                    }
                    building = false;
                    acc = 0;
                }
                _ => {
                    building = true;
                    acc = acc * 10 + c.to_digit(10).unwrap() as u8;
                }
            }
        }
        Number {
            values,
            depth: depths,
        }
    }

    fn add(&self, other: &Number) -> Number {
        Number {
            values: self
                .values
                .iter()
                .chain(other.values.iter())
                .copied()
                .collect_vec(),
            depth: self
                .depth
                .iter()
                .chain(other.depth.iter())
                .map(|d| d + 1)
                .collect_vec(),
        }
    }

    fn explode(&self) -> Option<Number> {
        for i in 0..self.values.len() {
            assert!(self.depth[i] <= 4);
            if self.depth[i] == 4 {
                let mut new_values = self.values.clone();
                let mut new_depth = self.depth.clone();
                if let Some(left) = i.checked_sub(1).and_then(|j| new_values.get_mut(j)) {
                    *left += self.values[i];
                }
                if let Some(right) = new_values.get_mut(i + 2) {
                    *right += self.values[i + 1];
                }
                new_values[i] = 0;
                new_values.remove(i + 1);
                new_depth[i] -= 1;
                new_depth.remove(i + 1);
                return Some(Number {
                    values: new_values,
                    depth: new_depth,
                });
            }
        }
        None
    }

    fn split(&self) -> Option<Number> {
        for i in 0..self.values.len() {
            if self.values[i] >= 10 {
                let mut new_values = self.values.clone();
                let mut new_depth = self.depth.clone();
                let left = self.values[i] / 2;
                let right = left + (self.values[i] % 2);
                new_values[i] = left;
                new_values.insert(i + 1, right);
                new_depth[i] += 1;
                new_depth.insert(i + 1, new_depth[i]);
                return Some(Number {
                    values: new_values,
                    depth: new_depth,
                });
            }
        }
        None
    }

    fn reduce(&self) -> Number {
        loop {
            if let Some(new_value) = self.explode() {
                return new_value.reduce();
            } else if let Some(new_value) = self.split() {
                return new_value.reduce();
            } else {
                return self.clone();
            }
        }
    }

    fn magnitude(&self) -> u64 {
        let mut stack: Vec<(u64, u8)> = vec![];
        for (&value, &depth) in self.values.iter().zip(&self.depth) {
            let mut new_value = value as u64;
            let mut new_depth = depth;
            while !stack.is_empty() && new_depth <= stack.last().unwrap().1 {
                let (left, _) = stack.pop().unwrap();
                new_value = left * 3 + new_value * 2;
                if new_depth == 0 {
                    return new_value;
                }
                new_depth -= 1;
            }
            stack.push((new_value, new_depth));
        }
        panic!();
    }
}

fn part2(input: Vec<Number>) -> u64 {
  input.iter().cartesian_product(input.iter()).map(|(m,n)| {
    if m != n {
      m.add(n).reduce().magnitude()
    } else {
      0
    }
  }).max().unwrap()
}

#[cfg(test)]
mod test {
    use crate::util::read_file;

    use super::*;

    #[test]
    fn test_parser() {
        let n = Number::parse("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        assert_eq!(
            Number {
                values: vec![0, 7, 4, 15, 0, 13, 1, 1],
                depth: vec![3, 3, 2, 2, 3, 3, 1, 1]
            },
            n
        );
        dbg!(Number::parse("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn test_add() {
        let left = Number {
            values: vec![1, 2],
            depth: vec![0, 0],
        };
        let right = Number {
            values: vec![3, 4, 5],
            depth: vec![1, 1, 0],
        };
        let result = left.add(&right);
        assert_eq!(vec![1, 2, 3, 4, 5], result.values);
        assert_eq!(vec![1, 1, 2, 2, 1], result.depth);
    }

    #[test]
    fn test_explode1() {
        let input = Number {
            values: vec![9, 8, 1, 2, 3, 4],
            depth: vec![4, 4, 3, 2, 1, 0],
        };
        let output = input.explode().unwrap();
        assert_eq!(vec![0, 9, 2, 3, 4], output.values);
        assert_eq!(vec![3, 3, 2, 1, 0], output.depth);
    }

    #[test]
    fn test_explode2() {
        let input = Number {
            values: vec![6, 5, 4, 3, 2, 1],
            depth: vec![1, 2, 3, 4, 4, 0],
        };
        let output = input.explode().unwrap();
        assert_eq!(vec![6, 5, 7, 0, 3], output.values);
        assert_eq!(vec![1, 2, 3, 3, 0], output.depth);
    }

    #[test]
    fn test_split() {
        let input = Number {
            values: vec![0, 7, 4, 15, 0, 13, 1, 1],
            depth: vec![3, 3, 2, 2, 3, 2, 1, 1],
        };
        let output = input.split().unwrap();
        assert_eq!(vec![0, 7, 4, 7, 8, 0, 13, 1, 1], output.values);
        assert_eq!(vec![3, 3, 2, 3, 3, 3, 2, 1, 1], output.depth);
    }

    #[test]
    fn test_reduce() {
        let input = Number::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        dbg!(input.reduce());
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(143, Number::parse("[[1,2],[[3,4],5]]").magnitude());
        assert_eq!(
            1137,
            Number::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude()
        );
    }

    #[test]
    fn example_day18_part1() {
        let numbers = read_file("day18.example")
            .unwrap()
            .map(|line| Number::parse(line.as_str()));
        let answer = numbers.reduce(|m, n| m.add(&n).reduce()).unwrap();
        assert_eq!(4140, answer.magnitude());
    }

    #[test]
    fn exec_day18_part1() {
        let numbers = read_file("day18.txt")
            .unwrap()
            .map(|line| Number::parse(line.as_str()));
        let answer = numbers.reduce(|m, n| m.add(&n).reduce()).unwrap();
        println!("Day 18 Part 1 - {}", answer.magnitude());
    }

    #[test]
    fn example_day18_part2() {
        let numbers = read_file("day18.example")
            .unwrap()
            .map(|line| Number::parse(line.as_str()));
        assert_eq!(3993, part2(numbers.collect_vec()));
    }

    #[test]
    fn exec_day18_part2() {
        let numbers = read_file("day18.txt")
            .unwrap()
            .map(|line| Number::parse(line.as_str()));
        println!("Day 18 Part 2 - {}", part2(numbers.collect_vec()));
    }
}
