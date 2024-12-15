use std::fs;

pub fn solve() {
    let input = fs::read_to_string("src/day_1/input").unwrap();
    let total = 0;
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_1/test_input").unwrap();
        let total = 0;
        assert_eq!(total, 0);
    }
}
