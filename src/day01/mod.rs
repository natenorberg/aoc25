use std::fs;

pub fn part1(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let turns = parse_input(&input);
    let mut position = 50;
    let mut num_zeros = 0;
    turns.into_iter().for_each(|turn| {
        position = next_number(position, turn);
        if position == 0 {
            num_zeros += 1;
        }
    });
    num_zeros
}

// Data types =================================================================
#[derive(Debug)]
enum Turn {
    Left(i32),
    Right(i32),
}

// Logic ======================================================================
fn next_number(initial: i32, turn: Turn) -> i32 {
    match turn {
        Turn::Left(distance) => (100 + initial - distance) % 100,
        Turn::Right(distance) => (100 + initial + distance) % 100,
    }
}

// Parsing ====================================================================
fn parse_line(line: &str) -> Turn {
    let (direction, distance) = line.split_at(1);
    let distance: i32 = distance.parse().expect("Invalid number");

    match direction {
        "L" => Turn::Left(distance),
        "R" => Turn::Right(distance),
        _ => panic!("Invalid direction"),
    }
}

fn parse_input(input: &str) -> Vec<Turn> {
    let lines: Vec<&str> = input.lines().collect();
    let turns: Vec<Turn> = lines.iter().map(|l| parse_line(l)).collect();
    turns
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_number() {
        assert_eq!(next_number(50, Turn::Left(68)), 82);
        assert_eq!(next_number(82, Turn::Left(30)), 52);
        assert_eq!(next_number(52, Turn::Right(48)), 0);
        assert_eq!(next_number(0, Turn::Left(5)), 95);
        assert_eq!(next_number(95, Turn::Right(60)), 55);
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/day01/test-input.txt"), 3);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1("src/day01/input.txt"), 1031);
    }
}
