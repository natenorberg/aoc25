use std::fs;

pub fn part1(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let banks = parse_input(&input);
    banks
        .into_iter()
        .map(|b| get_joltiest_batteries(b, 2))
        .map(get_joltage)
        .sum()
}

pub fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let banks = parse_input(&input);
    banks
        .into_iter()
        .map(|b| get_joltiest_batteries(b, 12))
        .map(get_joltage)
        .sum()
}

// Data types =================================================================

// Logic ======================================================================
fn get_joltiest_batteries(bank: Vec<u64>, num_batteries: usize) -> Vec<u64> {
    let mut remaining = num_batteries;
    let mut start_idx: usize = 0;
    let mut batteries: Vec<u64> = Vec::new();

    while remaining > 0 {
        remaining -= 1;
        let (battery, idx_in_slice) =
            get_joltiest_battery(&bank[start_idx..bank.len() - remaining]);
        batteries.push(battery);
        start_idx += idx_in_slice + 1;
    }
    batteries
}

fn get_joltiest_battery(options: &[u64]) -> (u64, usize) {
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

fn get_joltage(batteries: Vec<u64>) -> u64 {
    let mut joltage = 0;
    let base: u64 = 10;
    for (i, battery) in batteries.iter().rev().enumerate() {
        joltage += battery * base.pow((i as u64).try_into().unwrap());
    }
    joltage
}

// Parsing ====================================================================
fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input.lines().map(parse_bank).collect()
}

fn parse_bank(input: &str) -> Vec<u64> {
    let batteries: Vec<u64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
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
    fn test_get_joltiest_2_batteries() {
        let input =
            fs::read_to_string("src/inputs/day03/test-input.txt").expect("Couln't read the file");
        let banks = parse_input(&input);

        assert_eq!(get_joltiest_batteries(banks[0].clone(), 2), vec![9, 8]);
        assert_eq!(get_joltiest_batteries(banks[1].clone(), 2), vec![8, 9]);
        assert_eq!(get_joltiest_batteries(banks[2].clone(), 2), vec![7, 8]);
        assert_eq!(get_joltiest_batteries(banks[3].clone(), 2), vec![9, 2]);
    }

    #[test]
    fn test_get_joltiest_12_batteries() {
        let input =
            fs::read_to_string("src/inputs/day03/test-input.txt").expect("Couln't read the file");
        let banks = parse_input(&input);

        assert_eq!(
            get_joltiest_batteries(banks[0].clone(), 12),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]
        );
        assert_eq!(
            get_joltiest_batteries(banks[1].clone(), 12),
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
        );
        assert_eq!(
            get_joltiest_batteries(banks[2].clone(), 12),
            vec![4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]
        );
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day03/test-input.txt"), 357);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day03/input.txt"), 17443);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2("src/inputs/day03/test-input.txt"), 3121910778619);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2("src/inputs/day03/input.txt"), 172167155440541);
    }
}
