use std::fs;

pub fn part1(filename: &str) -> i64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let ranges = parse_input(&input);
    let mut invalid_sum = 0;

    ranges.into_iter().for_each(|range| {
        let invalid_numbers = get_invalid_numbers(range);
        invalid_numbers.iter().for_each(|i| {
            invalid_sum += i;
        });
    });

    invalid_sum
}

// Data types =================================================================
struct Range {
    start: i64,
    end: i64,
}

// Logic ======================================================================
fn get_invalid_numbers(range: Range) -> Vec<i64> {
    let mut invalid: Vec<i64> = Vec::new();
    for i in range.start..=range.end {
        if !is_valid(i) {
            invalid.push(i)
        }
    }
    invalid
}

fn is_valid(number: i64) -> bool {
    let string = number.to_string();
    if !string.len().is_multiple_of(2) {
        true
    } else {
        let (before, after) = string.split_at(string.len() / 2);
        before != after
    }
}

// Parsing ====================================================================
fn parse_input(input: &str) -> Vec<Range> {
    let range_inputs: Vec<&str> = input.trim_end().split(",").collect();
    let ranges: Vec<Range> = range_inputs.iter().map(|r| parse_range(r)).collect();
    ranges
}

fn parse_range(input: &str) -> Range {
    let parts: Vec<&str> = input.split("-").collect();
    let start: i64 = parts[0].parse().expect("Invalid number");
    let end: i64 = parts[1].parse().expect("Invalid number");
    Range { start, end }
}

// Tests ======================================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input =
            fs::read_to_string("src/inputs/day02/test-input.txt").expect("Couln't read the file");
        let ranges = parse_input(&input);
        assert_eq!(ranges[1].start, 95);
        assert_eq!(ranges[1].end, 115);
    }

    #[test]
    fn test_is_valid() {
        assert!(!is_valid(11));
        assert!(is_valid(121));
        assert!(!is_valid(1010));
    }

    #[test]
    fn test_get_invalid_numbers() {
        assert_eq!(get_invalid_numbers(Range { start: 11, end: 22 }), [11, 22]);
        assert_eq!(
            get_invalid_numbers(Range {
                start: 95,
                end: 115
            }),
            [99]
        );
        assert_eq!(
            get_invalid_numbers(Range {
                start: 998,
                end: 1012
            }),
            [1010]
        );
        assert_eq!(
            get_invalid_numbers(Range {
                start: 1188511880,
                end: 1188511890
            }),
            [1188511885]
        );
        assert_eq!(
            get_invalid_numbers(Range {
                start: 1698522,
                end: 1698528
            }),
            []
        );
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day02/test-input.txt"), 1227775554);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day02/input.txt"), 38158151648);
    }
}
