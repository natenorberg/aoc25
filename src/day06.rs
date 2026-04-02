use core::panic;
use std::fs;

pub fn part1(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let problems = parse_input_pt1(&input);
    problems.iter().map(solve_problem).sum()
}

pub fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let problems = parse_input_pt2(&input);
    problems.iter().map(solve_problem).sum()
}

// Data Structures ============================================================
#[derive(Debug, PartialEq)]
enum Problem {
    Sum(Vec<u64>),
    Product(Vec<u64>),
}

// Logic ======================================================================
fn solve_problem(problem: &Problem) -> u64 {
    match problem {
        Problem::Sum(numbers) => numbers.iter().sum(),
        Problem::Product(numbers) => numbers.iter().product(),
    }
}

// Parsing ====================================================================
fn parse_input_pt1(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.lines().collect();
    let number_lines = &lines[..lines.len() - 1];
    let numbers = parse_numbers_pt1(number_lines);
    parse_problems_pt1(lines.last().unwrap(), &numbers)
}

fn parse_numbers_pt1(input: &[&str]) -> Vec<Vec<u64>> {
    input.iter().map(|line| parse_number_line(*line)).collect()
}

fn parse_number_line(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|num_string| num_string.parse().unwrap())
        .collect()
}

fn parse_problems_pt1(operator_input: &str, numbers: &[Vec<u64>]) -> Vec<Problem> {
    let mut problems = Vec::new();

    for (i, operator) in operator_input.split_whitespace().enumerate() {
        let mut problem_numbers = Vec::new();
        for j in 0..numbers.len() {
            problem_numbers.push(numbers[j][i])
        }

        match operator {
            "+" => {
                problems.push(Problem::Sum(problem_numbers));
            }
            "*" => {
                problems.push(Problem::Product(problem_numbers));
            }
            &_ => panic!("Unknown operator"),
        }
    }

    problems
}

// Parse the input into a list of problems, just like part 1
//
// This ignores the right-to-left direction because it gets the same answer.
// Both + and * have the commutative property so the numbers of a problem be in any order
// We sum the answers together, so the columns can be in any order too.
fn parse_input_pt2(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.lines().collect();
    let number_lines = &lines[..lines.len() - 1];
    let number_char_map = parse_char_map(number_lines);
    let numbers = parse_numbers_pt2(&number_char_map);
    parse_problems_pt2(lines.last().unwrap(), &numbers)
}

fn parse_char_map(input: &[&str]) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

fn parse_numbers_pt2(map: &[Vec<char>]) -> Vec<Vec<u64>> {
    let mut problems = Vec::new();
    let mut current_numbers: Vec<u64> = Vec::new();

    for j in 0..map[0].len() {
        let number_string: String = map.iter().enumerate().map(|(i, _)| map[i][j]).collect();
        if number_string.trim() == "" {
            // All spaces, move on to the next problem
            problems.push(current_numbers);
            current_numbers = Vec::new();
        } else {
            current_numbers.push(number_string.trim().parse().unwrap());
        }
    }

    // Push the last problem on
    if !current_numbers.is_empty() {
        problems.push(current_numbers);
    }

    problems
}

fn parse_problems_pt2(operator_input: &str, numbers: &[Vec<u64>]) -> Vec<Problem> {
    let mut problems = Vec::new();

    for (i, operator) in operator_input.split_whitespace().enumerate() {
        match operator {
            "+" => {
                problems.push(Problem::Sum(numbers[i].clone()));
            }
            "*" => {
                problems.push(Problem::Product(numbers[i].clone()));
            }
            &_ => panic!("Unknown operator"),
        }
    }

    problems
}

// Tests ======================================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input_pt1() {
        let input =
            fs::read_to_string("src/inputs/day06/test-input.txt").expect("Couln't read the file");
        let parsed = parse_input_pt1(&input);

        assert_eq!(
            parsed,
            vec![
                Problem::Product(vec![123, 45, 6]),
                Problem::Sum(vec![328, 64, 98]),
                Problem::Product(vec![51, 387, 215]),
                Problem::Sum(vec![64, 23, 314]),
            ]
        )
    }

    #[test]
    fn test_parse_input_pt2() {
        let input =
            fs::read_to_string("src/inputs/day06/test-input.txt").expect("Couln't read the file");
        let parsed = parse_input_pt2(&input);

        assert_eq!(
            parsed,
            vec![
                Problem::Product(vec![1, 24, 356]),
                Problem::Sum(vec![369, 248, 8]),
                Problem::Product(vec![32, 581, 175]),
                Problem::Sum(vec![623, 431, 4]),
            ]
        )
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day06/test-input.txt"), 4277556);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day06/input.txt"), 5361735137219);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2("src/inputs/day06/test-input.txt"), 3263827);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2("src/inputs/day06/input.txt"), 11744693538946);
    }
}
