use std::fs;

pub fn part1(filename: &str) -> i64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let ranges = parse_input(&input);
    sum_invalid_numbers(ranges, true)
}

pub fn part2(filename: &str) -> i64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let ranges = parse_input(&input);
    sum_invalid_numbers(ranges, false)
}

// Data types =================================================================
struct Range {
    start: i64,
    end: i64,
}

// Logic ======================================================================
fn sum_invalid_numbers(ranges: Vec<Range>, only_check_doubles: bool) -> i64 {
    let mut invalid_sum = 0;

    ranges.into_iter().for_each(|range| {
        let invalid_numbers = get_invalid_numbers(range, only_check_doubles);
        invalid_numbers.iter().for_each(|i| {
            invalid_sum += i;
        });
    });

    invalid_sum
}

fn get_invalid_numbers(range: Range, only_check_doubles: bool) -> Vec<i64> {
    let mut invalid: Vec<i64> = Vec::new();
    for i in range.start..=range.end {
        if !is_valid(i, only_check_doubles) {
            invalid.push(i)
        }
    }
    invalid
}

fn is_valid(number: i64, only_check_doubles: bool) -> bool {
    if only_check_doubles {
        !is_repeated(number, 2)
    } else {
        let length = number.to_string().len();
        for i in 2..=length {
            if is_repeated(number, i) {
                return false;
            }
        }
        true
    }
}

fn is_repeated(number: i64, num_times: usize) -> bool {
    let string = number.to_string();
    if !string.len().is_multiple_of(num_times) {
        false
    } else {
        let chunks = split_chunks(&string, string.len() / num_times);
        let first = chunks[0];
        chunks.into_iter().all(|chunk| chunk == first)
    }
}

fn split_chunks(input: &str, size: usize) -> Vec<&str> {
    let mut chunks = Vec::new();
    let mut remaining = input;
    while !remaining.is_empty() {
        let (before, after) = remaining.split_at(size);
        chunks.push(before);
        remaining = after;
    }
    chunks
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
        assert!(!is_valid(11, true));
        assert!(is_valid(121, true));
        assert!(!is_valid(1010, true));
    }

    #[test]
    fn test_get_invalid_numbers_part_1() {
        assert_eq!(
            get_invalid_numbers(Range { start: 11, end: 22 }, true),
            [11, 22]
        );
        assert_eq!(
            get_invalid_numbers(
                Range {
                    start: 95,
                    end: 115
                },
                true
            ),
            [99]
        );
        assert_eq!(
            get_invalid_numbers(
                Range {
                    start: 998,
                    end: 1012
                },
                true
            ),
            [1010]
        );
        assert_eq!(
            get_invalid_numbers(
                Range {
                    start: 1188511880,
                    end: 1188511890
                },
                true
            ),
            [1188511885]
        );
        assert_eq!(
            get_invalid_numbers(
                Range {
                    start: 1698522,
                    end: 1698528
                },
                true
            ),
            []
        );
    }

    #[test]
    fn test_get_invalid_numbers_part_2() {
        assert_eq!(
            get_invalid_numbers(Range { start: 11, end: 22 }, false),
            [11, 22]
        );
        assert_eq!(
            get_invalid_numbers(
                Range {
                    start: 95,
                    end: 115
                },
                false
            ),
            [99, 111]
        );
        assert_eq!(
            get_invalid_numbers(
                Range {
                    start: 998,
                    end: 1012
                },
                false
            ),
            [999, 1010]
        );
        assert_eq!(
            get_invalid_numbers(
                Range {
                    start: 1188511880,
                    end: 1188511890
                },
                false
            ),
            [1188511885]
        );
        assert_eq!(
            get_invalid_numbers(
                Range {
                    start: 1698522,
                    end: 1698528
                },
                false
            ),
            []
        );
        assert_eq!(
            get_invalid_numbers(
                Range {
                    start: 2121212118,
                    end: 2121212124
                },
                false
            ),
            [2121212121]
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

    #[test]
    fn part2_test_input() {
        assert_eq!(part2("src/inputs/day02/test-input.txt"), 4174379265);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2("src/inputs/day02/input.txt"), 45283684555);
    }
}
