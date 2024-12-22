use std::fs;

fn parse_compacted_to_array(input: &str) -> Vec<i32> {
    let compacted: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let buffer_size = compacted.iter().sum();

    let mut buffer = Vec::with_capacity(buffer_size);
    buffer.resize(buffer_size, -1);

    let mut current_buffer_i = 0;
    for (i, c) in compacted.iter().enumerate() {
        if i % 2 == 0 {
            for j in current_buffer_i..current_buffer_i + c {
                buffer[j] = i as i32 / 2;
            }
        }
        current_buffer_i += c;
    }

    buffer
}

fn defrag_buffer(mut buffer: Vec<i32>) -> Vec<i32> {
    let mut forwards = 0;
    let mut backwards = buffer.len() - 1;

    while forwards < backwards {
        match (buffer[forwards], buffer[backwards]) {
            (-1, i) if i != -1 => {
                buffer.swap(forwards, backwards);
                forwards += 1;
                backwards -= 1;
            }
            (-1, -1) => {
                backwards -= 1;
            }
            _ => {
                forwards += 1;
            }
        }
    }

    buffer
}

fn calculate_checksum(buffer: Vec<i32>) -> usize {
    buffer
        .into_iter()
        .filter(|c| *c != -1)
        .enumerate()
        .map(|(i, c)| i * c as usize)
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_9/input").unwrap();
    let buffer = parse_compacted_to_array(&input);
    let defragged = defrag_buffer(buffer);
    let total = calculate_checksum(defragged);
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = fs::read_to_string("src/day_9/test_input").unwrap();
        let buffer = parse_compacted_to_array(&input);
        let defragged = defrag_buffer(buffer);
        let total = calculate_checksum(defragged);
        assert_eq!(total, 1928);
    }
}
