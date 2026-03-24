use std::fs;

enum Turn {
    Left(i32),
    Right(i32),
}

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
    // Probably a way to map for this, but
    let turns: Vec<Turn> = lines.iter().map(|l| parse_line(l)).collect();
    turns
}

pub fn part1() -> i32 {
    let input = fs::read_to_string("src/day01/test-input.txt").expect("Couln't read the file");
    let turns = parse_input(&input);
    println!("{}", turns.len());
    0
}
