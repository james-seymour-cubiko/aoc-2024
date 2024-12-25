use std::fs;

fn parse_input(input: &str) {}

pub fn solve() {
    let input = fs::read_to_string("src/day_17/input").unwrap();
    let _ = parse_input(&input);
    let total = 0;
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_17/test_input").unwrap();
        let _ = parse_input(&input);
        let total = 0;
        assert_eq!(total, 0);
    }
}
