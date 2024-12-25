use std::{collections::HashSet, fs};

use regex::Regex;

#[derive(Debug)]
struct Machine {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32),
}

const EXTRACT_NUMBERS: &str = r"(\d+).*?(\d+)";

fn parse_input(input: &str) -> Vec<Machine> {
    let extract_numbers_re = Regex::new(EXTRACT_NUMBERS).unwrap();

    input
        .split("\n\n")
        .map(|m| {
            let mut numbers = m.lines().map(|l| {
                extract_numbers_re
                    .captures_iter(l)
                    .map(|c| c.extract())
                    .map(|(_, [i, j])| (i.parse::<u32>().unwrap(), j.parse::<u32>().unwrap()))
                    .next()
                    .unwrap()
            });

            Machine {
                button_a: numbers.next().unwrap(),
                button_b: numbers.next().unwrap(),
                prize: numbers.next().unwrap(),
            }
        })
        .collect()
}

fn try_all_possibilities(machine: Machine) -> u32 {
    let mut prize_wins = HashSet::new();

    for a_presses in 0..101 {
        for b_presses in 0..101 {
            let result = (
                machine.button_a.0 * a_presses + machine.button_b.0 * b_presses,
                machine.button_a.1 * a_presses + machine.button_b.1 * b_presses,
            );

            if result == machine.prize {
                prize_wins.insert((a_presses, b_presses));
            }
        }
    }

    prize_wins
        .into_iter()
        .map(|(a_presses, b_presses)| a_presses * 3 + b_presses)
        .min()
        .unwrap_or(0)
}

fn min_required_tokens(machines: Vec<Machine>) -> u32 {
    machines.into_iter().map(|m| try_all_possibilities(m)).sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_13/input").unwrap();
    let machines = parse_input(&input);
    let total = min_required_tokens(machines);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_13/test_input").unwrap();
        let machines = parse_input(&input);
        let total = min_required_tokens(machines);
        assert_eq!(total, 480);
    }
}
