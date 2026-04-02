use core::panic;
use std::fs;

pub fn part1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let lines: Vec<&str> = input.lines().collect();
    get_total_splits(&lines)
}

pub fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    0
}

// Data Structures ============================================================

// Logic ======================================================================
fn get_total_splits(lines: &[&str]) -> u32 {
    let mut total_splits = 0;
    let start_beam = lines[0].find('S').unwrap();
    let mut beams = vec![start_beam];

    for line in &lines[1..lines.len() - 1] {
        let (next_beams, splits) = get_next_beams(&beams, line);
        beams = next_beams;
        total_splits += splits;
    }

    total_splits
}

fn get_next_beams(beams: &[usize], line: &str) -> (Vec<usize>, u32) {
    let mut next = Vec::new();
    let mut splits = 0;

    beams
        .iter()
        .for_each(|beam| match line.chars().nth(*beam).unwrap() {
            '.' => next.push(*beam),
            '^' => {
                if *beam > 0 {
                    next.push(*beam - 1);
                }
                if *beam < line.len() - 1 {
                    next.push(*beam + 1);
                }
                splits += 1;
            }
            _ => panic!("Unknown character"),
        });

    next.sort();
    next.dedup();
    (next, splits)
}

// Parsing ====================================================================
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// Tests ======================================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_next_beams() {
        let input =
            fs::read_to_string("src/inputs/day07/test-input.txt").expect("Couln't read the file");
        let lines: Vec<&str> = input.lines().collect();

        assert_eq!(get_next_beams(&[7], lines[1]), (vec![7], 0));
        assert_eq!(get_next_beams(&[7], lines[2]), (vec![6, 8], 1));
        assert_eq!(get_next_beams(&[6, 8], lines[3]), (vec![6, 8], 0));
        assert_eq!(get_next_beams(&[6, 8], lines[4]), (vec![5, 7, 9], 2));

        assert_eq!(get_next_beams(&[0], "^..."), (vec![1], 1));
        assert_eq!(get_next_beams(&[3], "...^"), (vec![2], 1));

        assert_eq!(get_next_beams(&[1], ".^.."), (vec![0, 2], 1));
        assert_eq!(get_next_beams(&[2], "..^."), (vec![1, 3], 1));
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day07/test-input.txt"), 21);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day07/input.txt"), 1687);
    }

    // #[test]
    // fn part2_test_input() {
    //     assert_eq!(part2("src/inputs/day07/test-input.txt"), 3263827);
    // }

    // #[test]
    // fn part2_real() {
    //     assert_eq!(part2("src/inputs/day07/input.txt"), 11744693538946);
    // }
}
