use regex::Regex;
use std::fs;

struct Grid {
    inner: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
}

fn is_match(a: char, b: char, c: char, d: char) -> bool {
    (a == 'X' && b == 'M' && c == 'A' && d == 'S') || (a == 'S' && b == 'A' && c == 'M' && d == 'X')
}

fn count_horizontal(grid: &Grid) -> usize {
    (0..grid.n_cols - 3)
        .map(|col| {
            (0..grid.n_rows)
                .map(|row| {
                    is_match(
                        grid.inner[row][col],
                        grid.inner[row][col + 1],
                        grid.inner[row][col + 2],
                        grid.inner[row][col + 3],
                    ) as usize
                })
                .sum::<usize>()
        })
        .sum()
}

fn count_vertical(grid: &Grid) -> usize {
    (0..grid.n_cols)
        .map(|col| {
            (0..grid.n_rows - 3)
                .map(|row| {
                    is_match(
                        grid.inner[row][col],
                        grid.inner[row + 1][col],
                        grid.inner[row + 2][col],
                        grid.inner[row + 3][col],
                    ) as usize
                })
                .sum::<usize>()
        })
        .sum()
}

fn count_diagonal(grid: &Grid) -> usize {
    // -ve diagonal
    (0..grid.n_cols - 3)
        .map(|col| {
            (0..grid.n_rows - 3)
                .map(|row| {
                    is_match(grid.inner[row][col],
                         grid.inner[row + 1][col + 1],
                         grid.inner[row + 2][col + 2],
                         grid.inner[row + 3][col + 3])
                     as usize
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        // +ve diagonal
        + (0..grid.n_cols - 3)
            .map(|col| {
                (3..grid.n_rows)
                    .map(|row| {
                        is_match(grid.inner[row][col],
                             grid.inner[row - 1][col + 1],
                             grid.inner[row - 2][col + 2],
                             grid.inner[row - 3][col + 3]) as usize
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
}

fn parse_grid(input: &str) -> Grid {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let row = line.chars().collect();
        grid.push(row);
    }

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    Grid {
        inner: grid,
        n_rows,
        n_cols,
    }
}

fn total_xmas_instances(grid: Grid) -> usize {
    let h = count_horizontal(&grid);
    let v = count_vertical(&grid);
    let d = count_diagonal(&grid);

    h + v + d
}

pub fn solve() {
    let input = fs::read_to_string("src/day_4/input").unwrap();
    let grid = parse_grid(&input);
    let total = total_xmas_instances(grid);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let test_input = fs::read_to_string("src/day_4/test_input").unwrap();
        let grid = parse_grid(&test_input);
        let total = total_xmas_instances(grid);
        assert_eq!(total, 18)
    }
}
