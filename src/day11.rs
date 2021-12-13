use crate::util::read_lines;

type Puzzle = Vec<Vec<u32>>;

pub fn parse_puzzle(file: &str) -> Puzzle {
    read_lines(file, |line: String| -> Option<Vec<u32>> {
        Some(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
    })
    .unwrap()
    .collect()
}

fn update_and_get(
    input: &mut Puzzle,
    x: Option<usize>,
    y: Option<usize>,
    f: fn(u32) -> u32,
) -> Option<u32> {
    let line = input.get_mut(y?)?;
    let cell = line.get_mut(x?)?;
    *cell = f(*cell);
    Some(*cell)
}

fn add1(input: Option<usize>) -> Option<usize> {
    input.and_then(|n| n.checked_add(1))
}

fn minus1(input: Option<usize>) -> Option<usize> {
    input.and_then(|n| n.checked_sub(1))
}

fn step(input: &mut Puzzle) -> u32 {
    let mut flashes = 0;

    let mut stack: Vec<(Option<usize>, Option<usize>)> = vec![];
    for x in (0..10).map(Some) {
        for y in (0..10).map(Some) {
            let cell = update_and_get(input, x, y, |n| n + 1).unwrap();
            assert!(cell <= 10);
            if cell == 10 {
                stack.push((x, y));
            }
        }
    }

    while let Some((x, y)) = stack.pop() {
        flashes += 1;
        [
            (minus1(x), minus1(y)),
            ((x), minus1(y)),
            (add1(x), minus1(y)),
            (minus1(x), y),
            (add1(x), y),
            (minus1(x), add1(y)),
            ((x), add1(y)),
            (add1(x), add1(y)),
        ]
        .iter()
        .for_each(|(x, y)| {
            if let Some(cell) = update_and_get(input, *x, *y, |n| n + 1) {
                if cell == 10 {
                    stack.push((*x, *y));
                }
            }
        });
    }

    for x in (0..10).map(Some) {
        for y in (0..10).map(Some) {
            update_and_get(input, x, y, |n| if n > 9 { 0 } else { n });
        }
    }

    flashes
}

pub fn part1(input: &mut Puzzle) -> u32 {
    (0..100).map(|_| step(input)).sum()
}

pub fn part2(input: &mut Puzzle) -> u32 {
    1 + (0..1000)
        .map(|_| step(input))
        .take_while(|n| *n != 100)
        .count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_day11_part1() {
        let mut input = parse_puzzle("day11.example");
        assert_eq!(1656, part1(&mut input));
    }

    #[test]
    fn exec_day11_part1() {
        let mut input = parse_puzzle("day11.txt");
        println!("Day 11 Part 1 - {}", part1(&mut input));
    }

    #[test]
    fn example_day11_part2() {
        let mut input = parse_puzzle("day11.example");
        assert_eq!(195, part2(&mut input));
    }

    #[test]
    fn exec_day11_part2() {
        let mut input = parse_puzzle("day11.txt");
        println!("Day 11 Part 2 - {}", part2(&mut input));
    }
}
