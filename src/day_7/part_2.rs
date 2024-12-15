use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
struct Equation {
    result: usize,
    terms: Vec<usize>,
}

fn parse_equations(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let (result, terms_str) = l.split_once(": ").unwrap();

            let mut terms: Vec<usize> = terms_str
                .split(" ")
                .map(|t| t.parse::<usize>().unwrap())
                .collect();

            terms.reverse();

            Equation {
                result: result.parse::<usize>().unwrap(),
                terms,
            }
        })
        .collect()
}

fn consume_equation(equation: Equation, desired_result: usize) -> bool {
    if equation.terms.len() == 0 {
        return equation.result == desired_result;
    }

    let mut try_mult_equation = equation.clone();
    let next_term = try_mult_equation.terms.pop().unwrap();
    try_mult_equation.result *= next_term;

    let mut try_add_equation = equation.clone();
    let next_term = try_add_equation.terms.pop().unwrap();
    try_add_equation.result += next_term;

    let mut try_concat_equation = equation.clone();
    let next_term = try_concat_equation.terms.pop().unwrap();
    try_concat_equation.result = format!("{}{}", try_concat_equation.result, next_term)
        .parse::<usize>()
        .unwrap();

    consume_equation(try_mult_equation, desired_result)
        || consume_equation(try_add_equation, desired_result)
        || consume_equation(try_concat_equation, desired_result)
}

fn is_equation_solvable(mut equation: Equation) -> bool {
    let desired_result = equation.result;
    equation.result = equation.terms.pop().unwrap();
    consume_equation(equation, desired_result)
}

fn solvable_total(equations: Vec<Equation>) -> usize {
    equations
        .into_iter()
        .map(|e| {
            let result = e.result;

            if is_equation_solvable(e) {
                result
            } else {
                0
            }
        })
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_7/input").unwrap();
    let equations = parse_equations(&input);
    let total = solvable_total(equations);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_7/test_input").unwrap();
        let equations = parse_equations(&input);
        let total = solvable_total(equations);
        assert_eq!(total, 11387);
    }
}
