use std::{
    collections::{HashSet, VecDeque},
    fs,
};

type Pos = (i32, i32);

struct Garden {
    n_rows: usize,
    n_cols: usize,
    inner: Vec<Vec<char>>,
    explored: HashSet<Pos>,
    total_price: usize,
}

fn parse_input(input: &str) -> Garden {
    let n_rows = input.lines().count();
    let n_cols = input.lines().next().expect("should contain 1 line").len();

    let inner = input.lines().map(|c| c.chars().collect()).collect();

    Garden {
        n_rows,
        n_cols,
        inner,
        explored: HashSet::new(),
        total_price: 0,
    }
}

fn in_bounds(pos: &Pos, n_rows: usize, n_cols: usize) -> bool {
    (0..n_rows as i32).contains(&pos.0) && (0..n_cols as i32).contains(&pos.1)
}

fn explore_region(mut garden: Garden, start: Pos) -> Garden {
    let mut areas_explored: HashSet<Pos> = HashSet::new();
    let mut perimeter_length = 0;

    let mut queue: VecDeque<Pos> = VecDeque::new();
    queue.push_back(start);

    while queue.len() != 0 {
        let pos = queue
            .pop_front()
            .expect("queue should have at least one element");

        areas_explored.insert(pos);

        for (i_offset, j_offset) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_pos = (pos.0 + i_offset, pos.1 + j_offset);

            if areas_explored.contains(&next_pos) || queue.contains(&next_pos) {
                continue;
            }

            if !in_bounds(&next_pos, garden.n_rows, garden.n_cols) {
                perimeter_length += 1;
                continue;
            }

            if garden.inner[pos.0 as usize][pos.1 as usize]
                != garden.inner[next_pos.0 as usize][next_pos.1 as usize]
            {
                println!("border - {:?}, {:?}", pos, next_pos);
                perimeter_length += 1;
                continue;
            }

            queue.push_back(next_pos);
        }
    }

    garden.total_price += areas_explored.len() * perimeter_length;
    garden.explored.extend(areas_explored);
    garden
}

fn explore_garden(mut garden: Garden) -> usize {
    for i in 0..garden.n_rows {
        for j in 0..garden.n_cols {
            let start = (i as i32, j as i32);
            if !garden.explored.contains(&start) {
                garden = explore_region(garden, start);
            }
        }
    }

    garden.total_price
}

pub fn solve() {
    let input = fs::read_to_string("src/day_12/input").unwrap();
    let garden = parse_input(&input);
    let total = explore_garden(garden);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_12/test_input").unwrap();
        let garden = parse_input(&input);
        let total = explore_garden(garden);
        assert_eq!(total, 1930);
    }
}
