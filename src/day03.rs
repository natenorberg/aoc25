use std::fs;

pub fn part1(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let banks = parse_input(&input);
    banks
        .into_iter()
        .map(get_joltiest_batteries)
        .map(get_joltage)
        .sum()
}

pub fn part2(filename: &str) -> i64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    0
}

// Data types =================================================================

// Logic ======================================================================
fn get_joltiest_batteries(bank: Vec<i32>) -> (i32, i32) {
    let (first, first_idx) = get_joltiest_battery(&bank[..bank.len() - 1]);
    let (second, _) = get_joltiest_battery(&bank[first_idx + 1..]);
    (first, second)
}

fn get_joltiest_battery(options: &[i32]) -> (i32, usize) {
    let mut joltage = 0;
    let mut index: usize = 0;

    for (i, battery) in options.iter().enumerate() {
        if *battery > joltage {
            joltage = *battery;
            index = i;
        }
    }
    (joltage, index)
}

fn get_joltage(batteries: (i32, i32)) -> i32 {
    (batteries.0 * 10) + batteries.1
}

// Parsing ====================================================================
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(parse_bank).collect()
}

fn parse_bank(input: &str) -> Vec<i32> {
    let batteries: Vec<i32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    batteries
}

// Tests ======================================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input =
            fs::read_to_string("src/inputs/day03/test-input.txt").expect("Couln't read the file");
        let banks = parse_input(&input);
        assert_eq!(banks[0].len(), 15);
        assert_eq!(banks[0][0], 9);
        assert_eq!(banks[2][2], 4);
    }

    #[test]
    fn test_get_joltiest_batteries() {
        let input =
            fs::read_to_string("src/inputs/day03/test-input.txt").expect("Couln't read the file");
        let banks = parse_input(&input);

        assert_eq!(get_joltiest_batteries(banks[0].clone()), (9, 8));
        assert_eq!(get_joltiest_batteries(banks[1].clone()), (8, 9));
        assert_eq!(get_joltiest_batteries(banks[2].clone()), (7, 8));
        assert_eq!(get_joltiest_batteries(banks[3].clone()), (9, 2));
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day03/test-input.txt"), 357);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day03/input.txt"), 17443);
    }

    // #[test]
    // fn part2_test_input() {
    //     assert_eq!(part2("src/inputs/day03/test-input.txt"), 4174379265);
    // }

    // #[test]
    // fn part2_real() {
    //     assert_eq!(part2("src/inputs/day03/input.txt"), 45283684555);
    // }
}
