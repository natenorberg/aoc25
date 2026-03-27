use std::fs;

pub fn part1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let map = parse_input(&input);
    get_accessible_rolls(&map).len() as u32
}

// TODO: Loop until nothing else accessible

pub fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    0
}
// Data Structures ============================================================
struct Location {
    x: usize,
    y: usize,
}

// Logic ======================================================================
fn remove_rolls(rolls: Vec<Location>, mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    rolls.into_iter().for_each(|location| {
        map[location.y][location.x] = '.';
    });
    map
}

fn get_accessible_rolls(map: &Vec<Vec<char>>) -> Vec<Location> {
    let mut accessible_rolls = Vec::new();

    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if *cell == '.' {
                continue;
            }
            let adjacent = get_num_adjacent_rolls(x as isize, y as isize, map);
            if adjacent < 4 {
                accessible_rolls.push(Location { x, y });
            }
        }
    }

    accessible_rolls
}

const ADJACENT_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_num_adjacent_rolls(x: isize, y: isize, map: &Vec<Vec<char>>) -> u32 {
    let mut adjacent_rolls = 0;

    for (offset_x, offset_y) in ADJACENT_OFFSETS {
        let check_x = x + offset_x;
        let check_y = y + offset_y;
        if check_y < 0 || check_y >= map.len() as isize {
            continue;
        }
        if check_x < 0 || check_x >= map[0].len() as isize {
            continue;
        }

        if map[check_y as usize][check_x as usize] == '@' {
            adjacent_rolls += 1
        }
    }

    adjacent_rolls
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
    fn test_get_num_adjacent_rolls() {
        let input =
            fs::read_to_string("src/inputs/day04/test-input.txt").expect("Couln't read the file");
        let map = parse_input(&input);

        assert_eq!(get_num_adjacent_rolls(2, 0, &map), 3);
        assert_eq!(get_num_adjacent_rolls(0, 1, &map), 3);
        assert_eq!(get_num_adjacent_rolls(4, 1, &map), 4);
        assert_eq!(get_num_adjacent_rolls(9, 2, &map), 4);
        assert_eq!(get_num_adjacent_rolls(4, 4, &map), 8);
        assert_eq!(get_num_adjacent_rolls(9, 7, &map), 4);
        assert_eq!(get_num_adjacent_rolls(0, 9, &map), 1);
    }

    #[test]
    fn test_remove_accessible_rolls() {
        let input =
            fs::read_to_string("src/inputs/day04/test-input.txt").expect("Couln't read the file");
        let map = parse_input(&input);
        let accesible_rolls = get_accessible_rolls(&map);
        let map = remove_rolls(accesible_rolls, map);

        let expected_input = ".......@..
.@@.@.@.@@
@@@@@...@@
@.@@@@..@.
.@.@@@@.@.
.@@@@@@@.@
.@.@.@.@@@
..@@@.@@@@
.@@@@@@@@.
....@@@...
";
        let expected_map = parse_input(&expected_input);
        assert_eq!(map, expected_map);

        // Run it again
        let accesible_rolls = get_accessible_rolls(&map);
        let map = remove_rolls(accesible_rolls, map);

        let expected_input = "..........
.@@.....@.
.@@@@...@@
..@@@@....
.@.@@@@...
..@@@@@@..
...@.@.@@@
..@@@.@@@@
..@@@@@@@.
....@@@...
";
        let expected_map = parse_input(&expected_input);
        assert_eq!(map, expected_map);
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day04/test-input.txt"), 13);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day04/input.txt"), 1356);
    }

    // #[test]
    // fn part2_test_input() {
    //     assert_eq!(part2("src/inputs/day04/test-input.txt"), 3121910778619);
    // }

    // #[test]
    // fn part2_real() {
    //     assert_eq!(part2("src/inputs/day04/input.txt"), 172167155440541);
    // }
}
