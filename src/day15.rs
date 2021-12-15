use std::{
    collections::{HashMap, HashSet, BinaryHeap},
    iter::repeat, cmp::Ordering,
};

use itertools::Itertools;

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

#[derive(Copy, Clone, Debug)]
struct MinNode {
    distance: u32,
    node: Node,
}

impl PartialEq for MinNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for MinNode {

}

impl PartialOrd for MinNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

pub fn part1(input: Puzzle) -> u32 {
    let height = input.len();
    let width = input[0].len();
    let goal = (width - 1, height - 1);

    // Initialise
    let mut count = 0;
    let mut visited: HashSet<Node> = HashSet::new();
    let mut distances: HashMap<Node, u32> = HashMap::new();
    distances.insert((0, 0), 0);

    let mut visit_next: BinaryHeap<MinNode> = BinaryHeap::new();
    visit_next.push(MinNode { distance: 0, node: (0, 0)});

    while !visit_next.is_empty() {
        // let smallest_position = visit_next
        //     .iter()
        //     .position_min_by_key(|node| distances.get(node).unwrap_or(&u32::MAX))
        //     .unwrap();
        // let current = visit_next.remove(smallest_position);
        let current = visit_next.pop().unwrap().node;

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
                visit_next.push(MinNode { distance: new_distance, node: (x, y) });
            }
        }

        visited.insert(current);

        count += 1;
    }

    distances[&goal]
}

pub fn foo(cell: u32, index: usize) -> u32 {
    (cell - 1 + index as u32) % 9 + 1
}

pub fn part2(input: Puzzle) -> u32 {
    let wider_input: Puzzle = input
        .iter()
        .map(|line| {
            repeat(line)
                .take(5)
                .enumerate()
                .map(|(i, segment)| segment.iter().map(move |cell| foo(*cell, i)))
                .flatten()
                .collect()
        })
        .collect();

    let bigger_input: Puzzle = repeat(wider_input)
        .take(5)
        .enumerate()
        .map(|(i, segment)| -> Puzzle {
            segment
                .iter()
                .map(move |line| line.iter().map(|cell| foo(*cell, i)).collect())
                .collect()
        })
        .flatten()
        .collect();

    part1(bigger_input)
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

    #[test]
    fn example_day15_part2() {
        let input = parse_puzzle("day15.example");
        assert_eq!(315, part2(input));
    }

    #[test]
    fn exec_day15_part2() {
        let input = parse_puzzle("day15.txt");
        println!("Day 15 Part 2 - {}", part2(input));
    }
}
