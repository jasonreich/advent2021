use std::collections::{HashMap, HashSet};

use crate::util::read_lines;

type Puzzle = Vec<Vec<u32>>;

pub fn parse_puzzle(file: &str) -> Puzzle {
    read_lines(file, |line: String| -> Option<Vec<u32>> {
        Some(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
    })
    .unwrap()
    .collect()
}

type Node = (usize, usize);

pub fn neighbours((x, y): Node) -> impl Iterator<Item = Node> {
    let mut output = vec![(x + 1, y), (x, y + 1)];

    if x > 0 {
        output.push((x - 1, y));
    }

    if y > 0 {
        output.push((x, y - 1));
    }

    output.into_iter()
}

pub fn part1(input: Puzzle) -> u32 {
    let height = input.len();
    let width = input[0].len();
    let goal = (width - 1, height - 1);

    // Initialise
    let mut visited: HashSet<Node> = HashSet::new();
    let mut distances: HashMap<Node, u32> = HashMap::new();
    distances.insert((0, 0), 0);

    let mut visit_next: Vec<Node> = Vec::new();
    visit_next.push((0, 0));

    while !visit_next.is_empty() {
        let current = visit_next.remove(0);

        if visited.contains(&current) {
            continue;
        }

        if current == goal {
            break;
        }

        let current_distance = distances[&current];

        for (x, y) in neighbours(current) {
            if visited.contains(&(x, y)) {
                continue;
            }

            if let Some(distance_to_node) = input.get(y).and_then(|line| line.get(x)) {
                let distance_from_start = current_distance + distance_to_node;
                let existing_distance = *distances.get(&(x, y)).unwrap_or(&u32::MAX);
                let new_distance = distance_from_start.min(existing_distance);
                distances.insert((x, y), new_distance);
                visit_next.push((x, y));
            }
        }

        visit_next.sort_by_cached_key(|node| distances.get(node).unwrap_or(&u32::MAX));
        visited.insert(current);
    }

    distances[&goal]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_day15_part1() {
        let input = parse_puzzle("day15.example");
        assert_eq!(40, part1(input));
    }

    #[test]
    fn exec_day15_part1() {
        let input = parse_puzzle("day15.txt");
        println!("Day 15 Part 1 - {}", part1(input));
    }
}
