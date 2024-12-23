use std::fs;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|c| c.parse().unwrap())
        .collect()
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::with_capacity(stones.len());

    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
            continue;
        }

        let stone_str = stone.to_string();

        if stone_str.len() % 2 == 0 {
            let mid = stone_str.len() / 2;
            new_stones.push(stone_str[..mid].parse().unwrap());
            new_stones.push(stone_str[mid..].parse().unwrap());
        } else {
            new_stones.push(stone * 2024);
        }
    }

    new_stones
}

fn blink_n(mut stones: Vec<u64>, n: usize) -> usize {
    for _ in 0..n {
        stones = blink(stones);
    }

    stones.len()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_11/input").unwrap();
    let stones = parse_input(&input);
    let total = blink_n(stones, 25);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_11/test_input").unwrap();
        let stones = parse_input(&input);
        let total = blink_n(stones, 25);
        assert_eq!(total, 55312);
    }
}
