use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs,
};

type Pos = (i32, i32);
type Pair = (Pos, Pos);

#[derive(Debug)]
struct Antennas {
    antennas: HashMap<char, HashSet<Pos>>,
    n_rows: usize,
    n_cols: usize,
}

fn parse_antennas(input: &str) -> Antennas {
    let mut antennas: HashMap<char, HashSet<Pos>> = HashMap::new();
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().expect("should contain 1 line").len();

    for (c, pos) in input.lines().enumerate().flat_map(|(row, l)| {
        l.chars().enumerate().filter_map(move |(col, c)| {
            if c != '.' {
                Some((c, (row as i32, col as i32)))
            } else {
                None
            }
        })
    }) {
        if let Some(locations) = antennas.get_mut(&c) {
            locations.insert(pos);
        } else {
            let mut locations = HashSet::new();
            locations.insert(pos);
            antennas.insert(c, locations);
        }
    }

    Antennas {
        antennas,
        n_rows,
        n_cols,
    }
}

fn all_pairs(locations: &HashSet<Pos>) -> HashSet<Pair> {
    let mut pairs = HashSet::new();

    for x in locations {
        for y in locations {
            if x != y {
                pairs.insert((x.clone(), y.clone()));
            }
        }
    }

    pairs
}

fn in_bounds(pos: &Pos, n_rows: usize, n_cols: usize) -> bool {
    (0..n_rows as i32).contains(&pos.0) && (0..n_cols as i32).contains(&pos.1)
}

fn antinodes_for_frequency(locations: &HashSet<Pos>, n_rows: usize, n_cols: usize) -> Vec<Pos> {
    let mut antinodes = Vec::new();

    for (a, b) in all_pairs(locations) {
        let (rel_row, rel_col) = (b.0 - a.0, b.1 - a.1);

        let required_iterations = max(
            n_rows / rel_row.abs() as usize,
            n_cols / rel_col.abs() as usize,
        );

        for i in (0..required_iterations) {
            let scale = i as i32;
            let a_antinode = (a.0 - scale * rel_row, a.1 - scale * rel_col);
            if in_bounds(&a_antinode, n_rows, n_cols) {
                antinodes.push(a_antinode);
            }

            let b_antinode = (b.0 + scale * rel_row, b.1 + scale * rel_col);
            if in_bounds(&b_antinode, n_rows, n_cols) {
                antinodes.push(b_antinode);
            }
        }
    }

    antinodes
}

fn total_antinodes(antennas: Antennas) -> usize {
    let mut antinodes = HashSet::new();

    for (_, locations) in antennas.antennas {
        antinodes.extend(antinodes_for_frequency(
            &locations,
            antennas.n_rows,
            antennas.n_cols,
        ))
    }

    antinodes.len()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_8/input").unwrap();
    let antennas = parse_antennas(&input);
    let total = total_antinodes(antennas);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_8/test_input").unwrap();
        let antennas = parse_antennas(&input);
        let total = total_antinodes(antennas);
        assert_eq!(total, 34);
    }
}
