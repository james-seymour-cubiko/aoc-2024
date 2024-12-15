use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Lab {
    n_rows: usize,
    n_cols: usize,
    start_pos: (i32, i32),
    obstructions: HashSet<(i32, i32)>,
}

fn parse_input(input: &str) -> Lab {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().expect("should contain 1 line").len();

    let start_pos = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| {
                if c == '^' {
                    Some((row as i32, col as i32))
                } else {
                    None
                }
            })
        })
        .next()
        .unwrap();

    let obstructions: HashSet<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| match c {
                '#' => Some((row as i32, col as i32)),
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

fn in_bounds(pos: &(i32, i32), n_rows: usize, n_cols: usize) -> bool {
    (0..n_rows as i32).contains(&pos.0) && (0..n_cols as i32).contains(&pos.1)
}

fn step(obstructions: &HashSet<(i32, i32)>, pos: (i32, i32), dir: usize) -> ((i32, i32), usize) {
    let next_pos = match dir {
        0 => (pos.0 - 1, pos.1),
        1 => (pos.0 + 1, pos.1),
        2 => (pos.0, pos.1 - 1),
        3 => (pos.0, pos.1 + 1),
        _ => panic!("unknown direction"),
    };

    if obstructions.contains(&next_pos) {
        let next_dir = match dir {
            0 => 3,
            1 => 2,
            2 => 0,
            3 => 1,
            _ => panic!("unknown direction"),
        };

        (pos, next_dir)
    } else {
        (next_pos, dir)
    }
}

fn is_lab_cyclic(lab: Lab) -> bool {
    let mut positions_seen: HashSet<((i32, i32), usize)> = HashSet::new();
    positions_seen.insert((lab.start_pos, 0));

    let mut current_pos = lab.start_pos;
    let mut current_dir = 0;

    while in_bounds(&current_pos, lab.n_rows, lab.n_cols) {
        positions_seen.insert((current_pos, current_dir));

        (current_pos, current_dir) = step(&lab.obstructions, current_pos, current_dir);

        if positions_seen.contains(&(current_pos, current_dir)) && current_pos != lab.start_pos {
            return true;
        }
    }

    false
}

fn total_cyclic_labs(original_lab: Lab) -> usize {
    (0..original_lab.n_rows)
        .map(|row| {
            (0..original_lab.n_cols)
                .map(|col| {
                    let mut new_obstructions = original_lab.obstructions.clone();
                    new_obstructions.insert((row as i32, col as i32));

                    let lab = Lab {
                        obstructions: new_obstructions,
                        ..original_lab
                    };

                    is_lab_cyclic(lab) as usize
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_6/input").unwrap();
    let lab = parse_input(&input);
    let total = total_cyclic_labs(lab);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let test_input = fs::read_to_string("src/day_6/test_input").unwrap();
        let lab = parse_input(&test_input);
        let total = total_cyclic_labs(lab);
        assert_eq!(total, 6);
    }
}
