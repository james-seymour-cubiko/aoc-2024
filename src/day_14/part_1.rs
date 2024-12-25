use std::{fs, ops::Range};

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

fn is_robot_in_quadrant(robot: &Robot, quadrant: &(Range<usize>, Range<usize>)) -> bool {
    quadrant.0.contains(&robot.pos.0) && quadrant.1.contains(&robot.pos.1)
}

fn robots_in_quadrants(bathroom: Bathroom) -> usize {
    let upper_left = (0..bathroom.n_rows / 2, 0..bathroom.n_cols / 2);
    let lower_left = (
        bathroom.n_rows / 2 + 1..bathroom.n_rows,
        0..bathroom.n_cols / 2,
    );
    let upper_right = (
        0..bathroom.n_rows / 2,
        bathroom.n_cols / 2 + 1..bathroom.n_cols,
    );
    let lower_right = (
        bathroom.n_rows / 2 + 1..bathroom.n_rows,
        bathroom.n_cols / 2 + 1..bathroom.n_cols,
    );

    let mut total: usize = 1;

    for quadrant in [upper_left, lower_left, upper_right, lower_right] {
        total *= bathroom
            .robots
            .iter()
            .map(|r| is_robot_in_quadrant(r, &quadrant) as usize)
            .sum::<usize>()
    }

    total
}

pub fn solve() {
    let input = fs::read_to_string("src/day_14/input").unwrap();
    let bathroom = parse_input(&input, 103, 101);
    let bathroom = step_n(bathroom, 100);
    let total = robots_in_quadrants(bathroom);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_14/test_input").unwrap();
        let bathroom = parse_input(&input, 7, 11);
        let bathroom = step_n(bathroom, 100);
        let total = robots_in_quadrants(bathroom);
        assert_eq!(total, 12);
    }
}
