use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Range,
};

type Pos = (usize, usize);

#[derive(Debug)]
struct Robot {
    pos: Pos,
    vel: (i32, i32),
}

#[derive(Debug)]
struct Bathroom {
    n_rows: usize,
    n_cols: usize,
    robots: Vec<Robot>,
}

fn parse_input(input: &str, n_rows: usize, n_cols: usize) -> Bathroom {
    let robots = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" ").unwrap();

            let (p_j, p_i) = p.strip_prefix("p=").unwrap().split_once(",").unwrap();
            let (v_j, v_i) = v.strip_prefix("v=").unwrap().split_once(",").unwrap();

            Robot {
                pos: (p_i.parse().unwrap(), p_j.parse().unwrap()),
                vel: (v_i.parse().unwrap(), v_j.parse().unwrap()),
            }
        })
        .collect();

    Bathroom {
        n_rows,
        n_cols,
        robots,
    }
}

fn step_robot(bathroom: &Bathroom, robot: &Robot) -> Robot {
    let new_pos = (
        ((robot.pos.0 as i32 + robot.vel.0 + bathroom.n_rows as i32) as usize % bathroom.n_rows),
        ((robot.pos.1 as i32 + robot.vel.1 + bathroom.n_cols as i32) as usize % bathroom.n_cols),
    );

    Robot {
        pos: new_pos,
        vel: robot.vel,
    }
}

fn step_all_robots(bathroom: &Bathroom) -> Bathroom {
    let new_robots = bathroom
        .robots
        .iter()
        .map(|r| step_robot(&bathroom, r))
        .collect();

    Bathroom {
        n_rows: bathroom.n_rows,
        n_cols: bathroom.n_cols,
        robots: new_robots,
    }
}

fn step_n(bathroom: Bathroom, n: usize) -> Bathroom {
    let mut current_bathroom = bathroom;

    for _ in 0..n {
        current_bathroom = step_all_robots(&current_bathroom);
    }

    current_bathroom
}

fn print_robot_config(bathroom: &Bathroom, step: usize) {
    let mut robot_positions: HashMap<Pos, usize> = HashMap::new();

    for pos in bathroom.robots.iter().map(|r| r.pos) {
        *robot_positions.entry(pos).or_insert(0) += 1;
    }

    let unique_robot_positions: HashSet<Pos> = robot_positions.keys().map(|k| *k).collect();

    for i in 0..bathroom.n_rows {
        let line: String = (0..bathroom.n_cols)
            .map(|j| {
                if unique_robot_positions.contains(&(i, j)) {
                    "R"
                } else {
                    "."
                }
            })
            .collect();

        if line.contains("RRRRRRRRRRRR") {
            println!("{}", step);
        }
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/day_14/input").unwrap();
    let mut bathroom = parse_input(&input, 103, 101);
    for step in 0..10403 {
        print_robot_config(&bathroom, step);
        bathroom = step_n(bathroom, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_14/test_input").unwrap();
        let bathroom = parse_input(&input, 7, 11);
        let bathroom = step_n(bathroom, 100);
        // print_robot_config(bathroom);
        assert_eq!(0, 0);
    }
}
