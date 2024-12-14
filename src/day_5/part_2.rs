use std::fs;

#[derive(Debug)]
struct Rule {
    pre: usize,
    post: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Update {
    pages: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let (x, y) = input
        .split_once("\n\n")
        .expect("should have exactly one instance of double newline");

    let rules = x
        .lines()
        .map(|r| {
            let (pre, post) = r.split_once("|").unwrap();
            Rule {
                pre: pre.parse::<usize>().unwrap(),
                post: post.parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Rule>>();

    let updates = y
        .lines()
        .map(|u| Update {
            pages: u
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        })
        .collect::<Vec<Update>>();

    (rules, updates)
}

fn non_matching_indexes(update: &Update, rule: &Rule) -> Option<(usize, usize)> {
    let i1 = update.pages.iter().position(|e| *e == rule.pre);
    let i2 = update.pages.iter().position(|e| *e == rule.post);

    let x = match (i1, i2) {
        (Some(p1), Some(p2)) => {
            if p1 < p2 {
                None
            } else {
                Some((p1, p2))
            }
        }
        _ => None,
    };
    x
}

fn make_update_valid(update: &mut Update, rules: &Vec<Rule>) {
    for rule in rules.iter() {
        if let Some((i1, i2)) = non_matching_indexes(&update, rule) {
            update.pages.swap(i1, i2);
            make_update_valid(update, rules);
        }
    }
}

fn sum_middle_elements(rules: Vec<Rule>, updates: Vec<Update>) -> usize {
    let mut total = 0;
    for mut update in updates.into_iter() {
        let original_update = update.clone();
        make_update_valid(&mut update, &rules);
        if update != original_update {
            total += update.pages[update.pages.len() / 2];
        }
    }

    total
}

pub fn solve() {
    let input = fs::read_to_string("src/day_5/input").unwrap();
    let (rules, updates) = parse_input(&input);
    let total = sum_middle_elements(rules, updates);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let test_input = fs::read_to_string("src/day_5/test_input").unwrap();
        let (rules, updates) = parse_input(&test_input);
        let total = sum_middle_elements(rules, updates);
        assert_eq!(total, 123);
    }
}
