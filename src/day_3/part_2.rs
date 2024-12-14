use regex::Regex;
use std::fs;

const MUL_EXPRESSION: &str = r"mul\((\d+),(\d+)\)|do\(()()\)|don't\(()()\)";

fn calculate_total(input: &str) -> u32 {
    let mul_regex = Regex::new(MUL_EXPRESSION).unwrap();

    let mut is_mul_enabled = true;

    mul_regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(full, [i1, i2])| match full {
            "do()" => {
                is_mul_enabled = true;
                0
            }
            "don't()" => {
                is_mul_enabled = false;
                0
            }
            _ => i1.parse::<u32>().unwrap() * i2.parse::<u32>().unwrap() * is_mul_enabled as u32,
        })
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_3/input").unwrap();
    let total = calculate_total(&input);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let total = calculate_total(&test_input);
        assert_eq!(total, 48)
    }
}
