use std::cmp::max;
use std::cmp::min;
use std::fs;

pub fn part1(filename: &str) -> usize {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let (ranges, ingredients) = parse_input(&input);

    ingredients
        .iter()
        .filter(|ingredient| is_fresh(**ingredient, &ranges[..]))
        .count()
}

pub fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let (ranges, _) = parse_input(&input);

    let ranges = collapse_ranges(ranges);
    count_possible_fresh_ingredients(&ranges)
}

// Data Structures ============================================================
#[derive(Debug, PartialEq)]
struct Range {
    from: u64,
    to: u64,
}

// Logic ======================================================================
fn is_fresh(ingredient: u64, ranges: &[Range]) -> bool {
    ranges
        .iter()
        .any(|range| ingredient >= range.from && ingredient <= range.to)
}

fn are_overlapping(first: &Range, second: &Range) -> bool {
    (second.from >= first.from && second.from <= first.to)
        || (first.from >= second.from && first.from <= second.to)
}

fn merge_ranges(first: &Range, second: &Range) -> Range {
    let new_from = min(first.from, second.from);
    let new_to = max(first.to, second.to);
    Range {
        from: new_from,
        to: new_to,
    }
}

fn collapse_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    let mut idx = 0;
    let mut compare_idx = 1;

    loop {
        let mut merged = false;

        loop {
            if compare_idx >= ranges.len() {
                break;
            }
            if are_overlapping(&ranges[idx], &ranges[compare_idx]) {
                // Expand first to cover both ranges and remove second
                ranges[idx] = merge_ranges(&ranges[idx], &ranges[compare_idx]);
                ranges.remove(compare_idx);
                merged = true;
            } else {
                compare_idx += 1;
            }
        }

        if merged {
            // Made changes. Keep idx the same and run the checks again
            compare_idx = idx + 1;
        } else {
            // Made it all the way through. Try with next idx
            idx += 1;
            compare_idx = idx + 1;
        }

        if idx >= ranges.len() {
            break;
        }
    }

    ranges
}

fn count_possible_fresh_ingredients(ranges: &[Range]) -> u64 {
    let mut count = 0;
    ranges
        .iter()
        .for_each(|range| count += range.to + 1 - range.from);

    count
}

// Parsing ====================================================================
fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let ranges = parts[0].lines().map(parse_range).collect();
    let ingredients = parts[1].lines().map(parse_ingredient).collect();

    (ranges, ingredients)
}

fn parse_range(input: &str) -> Range {
    let parts: Vec<&str> = input.split("-").collect();
    Range {
        from: parts[0].parse().unwrap(),
        to: parts[1].parse().unwrap(),
    }
}

fn parse_ingredient(input: &str) -> u64 {
    input.parse().unwrap()
}

// Tests ======================================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =
            fs::read_to_string("src/inputs/day05/test-input.txt").expect("Couln't read the file");
        let (ranges, ingredients) = parse_input(&input);

        assert_eq!(ranges[0], Range { from: 3, to: 5 });
        assert_eq!(ranges[1], Range { from: 10, to: 14 });

        assert_eq!(ingredients[0], 1);
        assert_eq!(ingredients[2], 8);
        assert_eq!(ingredients[5], 32);
    }

    #[test]
    fn test_are_overlapping() {
        assert!(are_overlapping(
            &Range { from: 16, to: 20 },
            &Range { from: 12, to: 18 }
        ));
        assert!(are_overlapping(
            &Range { from: 12, to: 18 },
            &Range { from: 16, to: 20 },
        ));
        assert!(are_overlapping(
            &Range { from: 10, to: 20 },
            &Range { from: 12, to: 18 },
        ));

        assert!(!are_overlapping(
            &Range { from: 3, to: 5 },
            &Range { from: 10, to: 14 },
        ));
        assert!(!are_overlapping(
            &Range { from: 10, to: 14 },
            &Range { from: 3, to: 5 },
        ));
    }

    #[test]
    fn test_merge_ranges() {
        assert_eq!(
            merge_ranges(&Range { from: 12, to: 18 }, &Range { from: 16, to: 20 }),
            Range { from: 12, to: 20 }
        );
    }

    #[test]
    fn test_collapse_ranges() {
        let input = vec![
            Range { from: 3, to: 5 },
            Range { from: 10, to: 14 },
            Range { from: 16, to: 20 },
            Range { from: 12, to: 18 },
        ];
        let expected = vec![Range { from: 3, to: 5 }, Range { from: 10, to: 20 }];

        assert_eq!(collapse_ranges(input), expected);
    }

    #[test]
    fn test_count_possible_fresh_ingredients() {
        assert_eq!(
            count_possible_fresh_ingredients(&[
                Range { from: 3, to: 5 },
                Range { from: 10, to: 20 }
            ]),
            14
        );
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day05/test-input.txt"), 3);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day05/input.txt"), 611);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2("src/inputs/day05/test-input.txt"), 14);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2("src/inputs/day05/input.txt"), 345995423801866);
    }
}
