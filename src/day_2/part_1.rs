use std::fs;

fn read_reports(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().expect("should parse element as int"))
                .collect()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn safe_reports(reports: Vec<Vec<usize>>) -> usize {
    reports
        .into_iter()
        .map(|r| {
            r.iter()
                .zip(r[1..].iter())
                .zip(r[2..].iter())
                .map(|((l1, l2), l3)| {
                    let range = 1..4;
                    let is_monotonic = (l1 < l2 && l2 < l3) || (l1 > l2 && l2 > l3);
                    let correct_difference =
                        range.contains(&l1.abs_diff(*l2)) && range.contains(&l2.abs_diff(*l3));

                    is_monotonic && correct_difference
                })
                .all(|l| l)
        })
        .filter(|r| *r)
        .count()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_2/input").unwrap();
    let reports = read_reports(input.as_str());
    let safe_reports = safe_reports(reports);
    println!("{}", safe_reports);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let test_input = fs::read_to_string("src/day_2/test_input").unwrap();
        let reports = read_reports(test_input.as_str());
        let safe_reports = safe_reports(reports);
        assert_eq!(safe_reports, 2);
    }
}
