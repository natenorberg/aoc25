use std::fs;

pub fn part1(filename: &str) -> i64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let tiles = parse_input(&input);
    get_largest_area(&tiles)
}

#[allow(unused)]
pub fn part2(filename: &str) -> u64 {
    0
}

// Data Structures ============================================================

#[derive(Debug, PartialEq)]
struct Tile {
    row: i64,
    col: i64,
}

// Logic ======================================================================

fn get_largest_area(tiles: &[Tile]) -> i64 {
    let mut largest = 0;

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let area = get_area(&tiles[i], &tiles[j]);
            if area > largest {
                largest = area;
            }
        }
    }

    largest
}

fn get_area(a: &Tile, b: &Tile) -> i64 {
    let (min_x, max_x) = (a.col.min(b.col), a.col.max(b.col));
    let width = max_x - min_x + 1;

    let (min_y, max_y) = (a.row.min(b.row), a.row.max(b.row));
    let height = max_y - min_y + 1;

    width * height
}

// Parsing ====================================================================

fn parse_input(input: &str) -> Vec<Tile> {
    input.lines().map(parse_tile).collect()
}

fn parse_tile(input: &str) -> Tile {
    let parts: Vec<&str> = input.split(",").collect();
    Tile {
        row: parts[1].parse().unwrap(),
        col: parts[0].parse().unwrap(),
    }
}

// Tests ======================================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_tile() {
        assert_eq!(parse_tile("123,456"), Tile { row: 456, col: 123 })
    }

    #[test]
    fn test_get_area() {
        assert_eq!(get_area(&parse_tile("2,5"), &parse_tile("9,7")), 24);
        assert_eq!(get_area(&parse_tile("7,1"), &parse_tile("11,7")), 35);
        assert_eq!(get_area(&parse_tile("7,3"), &parse_tile("2,3")), 6);
        assert_eq!(get_area(&parse_tile("2,5"), &parse_tile("11,1")), 50);
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day09/test-input.txt"), 50);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day09/input.txt"), 4749929916);
    }

    #[test]
    #[ignore]
    fn part2_test_input() {
        assert_eq!(part2("src/inputs/day09/test-input.txt"), 123);
    }

    #[test]
    #[ignore]
    fn part2_real() {
        assert_eq!(part2("src/inputs/day09/input.txt"), 12345);
    }
}
