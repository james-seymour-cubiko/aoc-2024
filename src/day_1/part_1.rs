use std::{fs, iter};

fn read_location_ids(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (location_1, location_2) = l.split_once("   ").unwrap();
            (
                location_1.parse::<usize>().unwrap(),
                location_2.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>()
}

fn calculate_total_location_id_distance(location_ids: Vec<(usize, usize)>) -> usize {
    let mut location_1_ids = location_ids
        .iter()
        .map(|(l1, _)| l1)
        .collect::<Vec<&usize>>();
    location_1_ids.sort();

    let mut location_2_ids = location_ids
        .iter()
        .map(|(_, l2)| l2)
        .collect::<Vec<&usize>>();
    location_2_ids.sort();

    iter::zip(location_1_ids, location_2_ids)
        .map(|(l1, l2)| l1.abs_diff(*l2))
        .sum::<usize>()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_1/input").unwrap();
    let location_ids = read_location_ids(input.as_str());
    let distance = calculate_total_location_id_distance(location_ids);
    println!("{}", distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let test_input = fs::read_to_string("src/day_1/test_input").unwrap();
        let location_ids = read_location_ids(test_input.as_str());
        let distance = calculate_total_location_id_distance(location_ids);
        assert_eq!(distance, 11);
    }
}
