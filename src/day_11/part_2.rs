use std::{collections::HashMap, fs};

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|c| c.parse().unwrap())
        .collect()
}

fn dfs(
    saved: &mut HashMap<(u64, usize), usize>,
    stone: u64,
    depth: usize,
    target_depth: usize,
) -> usize {
    if depth == target_depth {
        return 1;
    }

    if let Some(existing) = saved.get(&(stone, depth)) {
        return *existing;
    }

    let stone_str = stone.to_string();

    let result = if stone == 0 {
        dfs(saved, 1, depth + 1, target_depth)
    } else {
        if stone_str.len() % 2 == 0 {
            let mid = stone_str.len() / 2;
            let left = stone_str[..mid].parse().unwrap();
            let right = stone_str[mid..].parse().unwrap();

            dfs(saved, left, depth + 1, target_depth) + dfs(saved, right, depth + 1, target_depth)
        } else {
            dfs(saved, stone * 2024, depth + 1, target_depth)
        }
    };

    saved.insert((stone, depth), result);

    result
}

fn blink_n(stones: Vec<u64>, n: usize) -> usize {
    let mut saved = HashMap::new();
    stones.iter().map(|s| dfs(&mut saved, *s, 0, n)).sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_11/input").unwrap();
    let stones = parse_input(&input);
    let total = blink_n(stones, 75);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_11/test_input").unwrap();
        let stones = parse_input(&input);
        let total = blink_n(stones, 75);
        assert_eq!(total, 65601038650482);
    }
}
