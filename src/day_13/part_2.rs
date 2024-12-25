use std::fs;

use good_lp::{constraint, microlp, variable, variables, Solution, SolverModel};
use regex::Regex;

#[derive(Debug)]
struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
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
                    .map(|(_, [i, j])| (i.parse::<usize>().unwrap(), j.parse::<usize>().unwrap()))
                    .next()
                    .unwrap()
            });

            Machine {
                button_a: numbers.next().unwrap(),
                button_b: numbers.next().unwrap(),
                prize: {
                    let original_prize = numbers.next().unwrap();
                    (
                        original_prize.0 + 10000000000000,
                        original_prize.1 + 10000000000000,
                    )
                },
            }
        })
        .collect()
}

fn min_required_tokens_for_machine(machine: Machine) -> usize {
    let mut vars = variables!();
    let a_presses = vars.add(variable().min(0));
    let b_presses = vars.add(variable().min(0));

    let s = vars
        .minimise(3 * a_presses + b_presses)
        .using(microlp)
        .with(constraint!(
            a_presses * machine.button_a.0 as f64 + b_presses * machine.button_b.0 as f64
                == machine.prize.0 as f64
        ))
        .with(constraint!(
            a_presses * machine.button_a.1 as f64 + b_presses * machine.button_b.1 as f64
                == machine.prize.1 as f64
        ))
        .solve();

    if let Ok(sol) = s {
        let a_presses_f64 = sol.value(a_presses);
        let b_presses_f64 = sol.value(b_presses);

        // filter out non-integer solutions
        if (a_presses_f64 - a_presses_f64.round()).abs() > 0.01
            || (b_presses_f64 - b_presses_f64.round()).abs() > 0.01
        {
            return 0;
        }

        3 * sol.value(a_presses).round() as usize + sol.value(b_presses).round() as usize
    } else {
        0
    }
}

fn min_required_tokens(machines: Vec<Machine>) -> usize {
    machines
        .into_iter()
        .map(|m| min_required_tokens_for_machine(m))
        .sum()
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
        assert_eq!(total, 875318608908);
    }
}
