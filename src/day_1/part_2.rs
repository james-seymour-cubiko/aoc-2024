use std::{collections::HashMap, fs, iter};

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
    let mut right_list_appearances: HashMap<usize, usize> = HashMap::new();

    for l2 in location_ids.iter().map(|(_, l2)| l2) {
        if let Some(apperances) = right_list_appearances.get_mut(l2) {
            *apperances += 1;
        } else {
            right_list_appearances.insert(*l2, 1);
        }
    }

    location_ids
        .iter()
        .map(|(l1, _)| {
            right_list_appearances
                .get(l1)
                .map(|appearances| *l1 * *appearances)
                .unwrap_or(0)
        })
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
        assert_eq!(distance, 31);
    }
}
