use std::fs;

pub fn part1(filename: &str) -> usize {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let (ranges, ingredients) = parse_input(&input);

    ingredients
        .iter()
        .filter(|ingredient| is_fresh(**ingredient, &ranges[..]))
        .count()
}

pub fn part2(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    0
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
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day05/test-input.txt"), 3);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day05/input.txt"), 611);
    }

    // #[test]
    // fn part2_test_input() {
    //     assert_eq!(part2("src/inputs/day05/test-input.txt"), 43);
    // }

    // #[test]
    // fn part2_real() {
    //     assert_eq!(part2("src/inputs/day05/input.txt"), 8713);
    // }
}
