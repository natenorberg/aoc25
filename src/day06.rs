use core::panic;
use std::fs;

pub fn part1(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let problems = parse_input(&input);
    problems.iter().map(solve_problem).sum()
}

pub fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    0
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
fn parse_input(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.lines().collect();
    let number_lines = &lines[..lines.len() - 1];
    let numbers = parse_numbers(number_lines);
    parse_problems(lines.last().unwrap(), &numbers)
}

fn parse_numbers(input: &[&str]) -> Vec<Vec<u64>> {
    input.iter().map(|line| parse_number_line(*line)).collect()
}

fn parse_number_line(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|num_string| num_string.parse().unwrap())
        .collect()
}

fn parse_problems(operator_input: &str, numbers: &[Vec<u64>]) -> Vec<Problem> {
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

// Tests ======================================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =
            fs::read_to_string("src/inputs/day06/test-input.txt").expect("Couln't read the file");
        let parsed = parse_input(&input);

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
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day06/test-input.txt"), 4277556);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day06/input.txt"), 5361735137219);
    }

    // #[test]
    // fn part2_test_input() {
    //     assert_eq!(part2("src/inputs/day06/test-input.txt"), 14);
    // }

    // #[test]
    // fn part2_real() {
    //     assert_eq!(part2("src/inputs/day06/input.txt"), 345995423801866);
    // }
}
