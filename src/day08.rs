use std::fs;

pub fn part1(filename: &str, num_connections: u32) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let boxes = parse_input(&input);
    0
}

pub fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    0
}

// Data Structures ============================================================
#[derive(Debug, PartialEq, Clone, Copy)]
struct Box {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Connection {
    a: Box,
    b: Box,
    distance: f64,
}

// Logic ======================================================================

fn build_connections(boxes: &[Box]) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();

    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            let a = boxes[i];
            let b = boxes[j];
            let distance = get_distance(&a, &b);
            connections.push(Connection { a, b, distance });
        }
    }

    connections.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    connections
}

fn get_distance(a: &Box, b: &Box) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)).sqrt()
}

// Parsing ====================================================================

fn parse_input(input: &str) -> Vec<Box> {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Box {
    let parts: Vec<&str> = input.split(",").collect();

    Box {
        x: parts[0].parse().unwrap(),
        y: parts[1].parse().unwrap(),
        z: parts[2].parse().unwrap(),
    }
}

// Tests ======================================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("162,817,812"),
            Box {
                x: 162.0,
                y: 817.0,
                z: 812.0,
            },
        );
    }

    fn get_box(x: i64, y: i64, z: i64) -> Box {
        Box {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    #[test]
    fn test_get_sort_distances() {
        let input =
            fs::read_to_string("src/inputs/day08/test-input.txt").expect("Couln't read the file");
        let boxes = parse_input(&input);
        let connections = build_connections(&boxes);

        assert_eq!(connections[0].a, get_box(162, 817, 812));
        assert_eq!(connections[0].b, get_box(425, 690, 689));

        assert_eq!(connections[1].a, get_box(162, 817, 812));
        assert_eq!(connections[1].b, get_box(431, 825, 988));
    }

    // #[test]
    // fn part1_test_input() {
    //     assert_eq!(part1("src/inputs/day08/test-input.txt", 10), 40);
    // }

    // #[test]
    // fn part1_real() {
    //     assert_eq!(part1("src/inputs/day08/input.txt", 1000), 5361735137219);
    // }

    // #[test]
    // fn part2_test_input() {
    //     assert_eq!(part2("src/inputs/day08/test-input.txt"), 3263827);
    // }

    // #[test]
    // fn part2_real() {
    //     assert_eq!(part2("src/inputs/day08/input.txt"), 11744693538946);
    // }
}
