use std::fs;

pub fn part1(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let turns = parse_input(&input);
    let mut position = 50;
    let mut num_zeros = 0;
    turns.into_iter().for_each(|turn| {
        (position, _) = next_number(position, turn);
        if position == 0 {
            num_zeros += 1;
        }
    });
    num_zeros
}

pub fn part2(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let turns = parse_input(&input);
    let mut position = 50;
    let mut num_zeros = 0;
    let mut new_zeros = 0;
    turns.into_iter().for_each(|turn| {
        (position, new_zeros) = next_number(position, turn);
        num_zeros += new_zeros;
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
fn next_number(initial: i32, turn: Turn) -> (i32, i32) {
    match turn {
        Turn::Left(distance) => {
            let extra_turns = distance / 100;
            let final_turn = distance % 100;
            let raw_position = initial - final_turn;
            let final_turn_touched_zero = initial != 0 && raw_position <= 0;
            let next_position = ((raw_position % 100) + 100) % 100;
            let turns = if final_turn_touched_zero {
                extra_turns + 1
            } else {
                extra_turns
            };
            (next_position, turns)
        }
        Turn::Right(distance) => {
            let extra_turns = distance / 100;
            let final_turn = distance % 100;
            let raw_position = initial + final_turn;
            let final_turn_touched_zero = initial != 0 && raw_position > 99;
            let next_position = ((raw_position % 100) + 100) % 100;
            let turns = if final_turn_touched_zero {
                extra_turns + 1
            } else {
                extra_turns
            };
            (next_position, turns)
            //let raw_position = initial + distance;
            //let next_position = raw_position % 100;
            //(next_position, raw_position / 100)
        }
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
        assert_eq!(next_number(50, Turn::Left(68)), (82, 1));
        assert_eq!(next_number(82, Turn::Left(30)), (52, 0));
        assert_eq!(next_number(52, Turn::Right(48)), (0, 1));
        assert_eq!(next_number(0, Turn::Left(5)), (95, 0));
        assert_eq!(next_number(95, Turn::Right(60)), (55, 1));
        assert_eq!(next_number(55, Turn::Left(55)), (0, 1));
        assert_eq!(next_number(0, Turn::Left(1)), (99, 0));
    }

    #[test]
    fn test_next_number_multiple_spins() {
        assert_eq!(next_number(50, Turn::Right(1000)), (50, 10));
        assert_eq!(next_number(1, Turn::Left(202)), (99, 3));
        assert_eq!(next_number(50, Turn::Left(1000)), (50, 10));
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/day01/test-input.txt"), 3);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1("src/day01/input.txt"), 1031);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2("src/day01/test-input.txt"), 6);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2("src/day01/input.txt"), 5831);
    }
}
