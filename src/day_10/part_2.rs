use std::{collections::HashSet, fs};

type Pos = (i32, i32);
type Pair = (Pos, Pos);

#[derive(Debug)]
struct Map {
    n_rows: usize,
    n_cols: usize,
    trailheads: Vec<Location>,
    inner: Vec<Vec<u32>>,
}

#[derive(Debug)]
struct Location {
    height: u32,
    pos: Pos,
    already_visited: Vec<Pos>,
}

fn parse_input(input: &str) -> Map {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().expect("should contain 1 line").len();

    let inner: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let trailheads: Vec<Location> = inner
        .iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.iter().enumerate().filter_map(move |(j, n)| match n {
                0 => Some(Location {
                    height: 0,
                    pos: (i as i32, j as i32),
                    already_visited: Vec::new(),
                }),
                _ => None,
            })
        })
        .collect();

    Map {
        n_rows,
        n_cols,
        trailheads,
        inner,
    }
}

fn in_bounds(pos: &Pos, n_rows: usize, n_cols: usize) -> bool {
    (0..n_rows as i32).contains(&pos.0) && (0..n_cols as i32).contains(&pos.1)
}

fn walk_trail(map: &Map, location: &Location) -> Vec<Vec<Pos>> {
    let mut unique_trails: Vec<Vec<Pos>> = Vec::new();

    for (i_offset, j_offset) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let new_pos = (location.pos.0 + i_offset, location.pos.1 + j_offset);

        if !in_bounds(&new_pos, map.n_rows, map.n_cols) {
            continue;
        }

        let new_height = map.inner[new_pos.0 as usize][new_pos.1 as usize];

        if location.height + 1 != new_height {
            continue;
        }

        if new_height == 9 {
            unique_trails.push(location.already_visited.clone());
        }

        let mut already_visited = location.already_visited.clone();
        already_visited.push(location.pos);

        let new_location = Location {
            height: new_height,
            pos: new_pos,
            already_visited,
        };

        unique_trails.extend(walk_trail(map, &new_location));
    }

    unique_trails
}

fn walk_all_trails(map: Map) -> usize {
    map.trailheads
        .iter()
        .map(|th| walk_trail(&map, &th).len())
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_10/input").unwrap();
    let map = parse_input(&input);
    let total = walk_all_trails(map);
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_10/test_input").unwrap();
        let map = parse_input(&input);
        let total = walk_all_trails(map);
        assert_eq!(total, 81);
    }
}
