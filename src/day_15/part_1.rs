use std::fs;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Warehouse {
    start: Pos,
    map: Vec<Vec<char>>,
    instructions: Vec<Direction>,
}

impl Warehouse {
    fn swap(self: &mut Self, pos_1: Pos, pos_2: Pos) {
        (self.map[pos_1.0][pos_1.1], self.map[pos_2.0][pos_2.1]) =
            (self.map[pos_2.0][pos_2.1], self.map[pos_1.0][pos_1.1]);
    }
}

fn parse_input(input: &str) -> Warehouse {
    let (m, i) = input.split_once("\n\n").unwrap();

    let mut start: Option<Pos> = None;

    let map = m
        .lines()
        .enumerate()
        .map(|(i, r)| {
            r.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '@' {
                        start = Some((i as usize, j as usize));
                    }
                    c
                })
                .collect()
        })
        .collect();

    let pos = start.unwrap();

    let instructions = i
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                i => panic!("unexpected instruction {}", i),
            })
        })
        .collect();

    Warehouse {
        start: pos,
        map,
        instructions,
    }
}

fn direction_offset(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn try_move_once(
    warehouse: &mut Warehouse,
    original_pos: Pos,
    current_pos: Option<Pos>,
    dir: Direction,
) -> bool {
    let offset: (i32, i32) = direction_offset(dir);
    let pos = current_pos.unwrap_or(original_pos);

    let next_pos = (
        (pos.0 as i32 + offset.0) as usize,
        (pos.1 as i32 + offset.1) as usize,
    );

    match warehouse.map[next_pos.0][next_pos.1] {
        '#' => false,
        '.' => {
            warehouse.swap(original_pos, next_pos);
            true
        }
        'O' => try_move_once(warehouse, original_pos, Some(next_pos), dir),
        _ => panic!("unexpected char"),
    }
}

fn move_robot_once(warehouse: &mut Warehouse, robot_pos: Pos, dir: Direction) -> Pos {
    let offset: (i32, i32) = direction_offset(dir);
    let robot_next_pos = (
        (robot_pos.0 as i32 + offset.0) as usize,
        (robot_pos.1 as i32 + offset.1) as usize,
    );

    let next_obj = warehouse.map[robot_next_pos.0][robot_next_pos.1];

    if next_obj == '#' {
        robot_pos
    } else if next_obj == '.' {
        warehouse.swap(robot_pos, robot_next_pos);
        robot_next_pos
    } else if try_move_once(warehouse, robot_next_pos, None, dir) {
        // Move robot if boxes have been pushed
        warehouse.swap(robot_pos, robot_next_pos);
        robot_next_pos
    } else {
        robot_pos
    }
}

fn execute_instructions(warehouse: &mut Warehouse) {
    let mut current_pos = warehouse.start;
    let instructions = &warehouse.instructions.clone();

    for i in instructions {
        current_pos = move_robot_once(warehouse, current_pos, *i);
    }
}

fn total_gps_score(warehouse: &Warehouse) -> usize {
    warehouse
        .map
        .iter()
        .enumerate()
        .map(|(i, r)| {
            r.iter()
                .enumerate()
                .map(|(j, c)| if *c == 'O' { 100 * i + j } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_15/input").unwrap();
    let mut warehouse = parse_input(&input);
    execute_instructions(&mut warehouse);
    let total = total_gps_score(&warehouse);
    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_15/test_input").unwrap();
        let mut warehouse = parse_input(&input);
        execute_instructions(&mut warehouse);
        let total = total_gps_score(&warehouse);
        assert_eq!(total, 10092);
    }
}
