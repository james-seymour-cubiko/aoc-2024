use std::fs;

struct Grid {
    inner: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
}

fn is_match(center: char, a: char, b: char, c: char, d: char) -> bool {
    center == 'A'
        && ((a == 'M' && c == 'S') || (a == 'S' && c == 'M'))
        && ((b == 'M' && d == 'S') || (b == 'S' && d == 'M'))
}

fn count_xmas(grid: &Grid) -> usize {
    (1..grid.n_cols - 1)
        .map(|col| {
            (1..grid.n_rows - 1)
                .map(|row| {
                    is_match(
                        grid.inner[row][col],
                        grid.inner[row - 1][col - 1],
                        grid.inner[row - 1][col + 1],
                        grid.inner[row + 1][col + 1],
                        grid.inner[row + 1][col - 1],
                    ) as usize
                })
                .sum::<usize>()
        })
        .sum()
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
    count_xmas(&grid)
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
        assert_eq!(total, 9)
    }
}
