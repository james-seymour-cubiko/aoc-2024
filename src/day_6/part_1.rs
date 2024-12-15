use std::{collections::HashSet, fs};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Lab {
    n_rows: usize,
    n_cols: usize,
    start_pos: (usize, usize),
    obstructions: HashSet<(usize, usize)>,
}

fn parse_input(input: &str) -> Lab {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().expect("should contain 1 line").len();

    let start_pos = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(col, c)| if c == '^' { Some((row, col)) } else { None })
        })
        .next()
        .unwrap();

    let obstructions: HashSet<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| match c {
                '#' => Some((row, col)),
                _ => None,
            })
        })
        .collect();

    Lab {
        n_rows,
        n_cols,
        start_pos,
        obstructions,
    }
}

fn in_bounds(pos: &(usize, usize), n_rows: usize, n_cols: usize) -> bool {
    (0..n_rows).contains(&pos.0) && (0..n_cols).contains(&pos.1)
}

fn step(
    obstructions: &HashSet<(usize, usize)>,
    pos: (usize, usize),
    dir: Direction,
) -> ((usize, usize), Direction) {
    let next_pos = match dir {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    };

    if obstructions.contains(&next_pos) {
        let next_dir = match dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };

        (pos, next_dir)
    } else {
        (next_pos, dir)
    }
}

fn walk_lab(lab: Lab) -> usize {
    let mut positions_seen: HashSet<(usize, usize)> = HashSet::new();
    positions_seen.insert(lab.start_pos);

    let mut current_pos = lab.start_pos;
    let mut current_dir = Direction::Up;

    while in_bounds(&current_pos, lab.n_rows, lab.n_cols) {
        positions_seen.insert(current_pos);
        (current_pos, current_dir) = step(&lab.obstructions, current_pos, current_dir);
    }

    positions_seen.len()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_6/input").unwrap();
    let lab = parse_input(&input);
    let total = walk_lab(lab);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let test_input = fs::read_to_string("src/day_6/test_input").unwrap();
        let lab = parse_input(&test_input);
        let total = walk_lab(lab);
        assert_eq!(total, 41);
    }
}
