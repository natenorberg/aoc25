use std::fs;

pub fn part1(filename: &str, num_connections: usize) -> u64 {
    let (_boxes, connections, mut circuits) = init(&filename);

    join_closest_circuits(num_connections, &mut circuits, &connections);
    circuits.sort_by(|a, b| b.box_ids.len().cmp(&a.box_ids.len()));
    get_part1_score(&circuits)
}

#[allow(unused)]
pub fn part2(filename: &str) -> u64 {
    // let input = fs::read_to_string(filename).expect("Couln't read the file");
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
    from_idx: usize,
    to_idx: usize,
    distance: f64,
}

#[derive(PartialEq, Clone)]
struct Circuit {
    box_ids: Vec<usize>,
}

impl Circuit {
    fn merge(&mut self, other: &Circuit) {
        self.box_ids.extend_from_slice(&other.box_ids);
    }

    fn has_box(&self, box_id: usize) -> bool {
        self.box_ids.contains(&box_id)
    }
}

// Logic ======================================================================

/*
 * The main logic. Join the circuits in the given connection
 */
fn connect(circuits: &mut Vec<Circuit>, connection: &Connection) {
    let a_idx = circuits
        .iter()
        .position(|c| c.has_box(connection.from_idx))
        .unwrap();
    let b_idx = circuits
        .iter()
        .position(|c| c.has_box(connection.to_idx))
        .unwrap();

    if a_idx == b_idx {
        return; // Circuits are the same. Don't do anything
    }

    // Different circuits. Merge B into A
    let b = circuits.swap_remove(b_idx);
    circuits[a_idx].merge(&b);
}

fn join_closest_circuits(n: usize, circuits: &mut Vec<Circuit>, connections: &[Connection]) {
    (0..n).for_each(|i| {
        let connection = &connections[i];
        connect(circuits, connection)
    });
}

fn build_connections(boxes: &[Box]) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();

    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            let a = boxes[i];
            let b = boxes[j];
            let distance = get_distance(&a, &b);
            connections.push(Connection {
                from_idx: i,
                to_idx: j,
                distance,
            });
        }
    }

    connections.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    connections
}

fn init_circuits(count: usize) -> Vec<Circuit> {
    (0..count).map(|i| Circuit { box_ids: vec![i] }).collect()
}

fn get_distance(a: &Box, b: &Box) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)).sqrt()
}

fn init(filename: &str) -> (Vec<Box>, Vec<Connection>, Vec<Circuit>) {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let boxes = parse_input(&input);
    let connections = build_connections(&boxes);
    let circuits = init_circuits(boxes.len());

    (boxes, connections, circuits)
}

// Scoring ====================================================================
fn get_part1_score(circuits: &[Circuit]) -> u64 {
    if !circuits.is_sorted_by(|a, b| a.box_ids.len() >= b.box_ids.len()) {
        panic!("Not sorted")
    }
    let lengths: Vec<u64> = circuits.iter().map(|c| c.box_ids.len() as u64).collect();
    lengths[0] * lengths[1] * lengths[2]
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
        let (boxes, connections, _circuits) = init("src/inputs/day08/test-input.txt");

        assert_eq!(boxes[connections[0].from_idx], get_box(162, 817, 812));
        assert_eq!(boxes[connections[0].to_idx], get_box(425, 690, 689));

        assert_eq!(boxes[connections[1].from_idx], get_box(162, 817, 812));
        assert_eq!(boxes[connections[1].to_idx], get_box(431, 825, 988));
    }

    #[test]
    fn test_merge_circuits() {
        let mut circuit1 = Circuit { box_ids: vec![0] };
        let mut circuit2 = Circuit { box_ids: vec![1] };

        circuit1.merge(&mut circuit2);
        assert_eq!(circuit1.box_ids, vec![0, 1])
    }

    #[test]
    fn test_join_closest_circuits() {
        let (_boxes, connections, mut circuits) = init("src/inputs/day08/test-input.txt");
        join_closest_circuits(10, &mut circuits, &connections);

        circuits.sort_by(|a, b| b.box_ids.len().cmp(&a.box_ids.len()));
        assert_eq!(circuits[0].box_ids.len(), 5);
        assert_eq!(circuits[1].box_ids.len(), 4);
        assert_eq!(circuits[2].box_ids.len(), 2);
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day08/test-input.txt", 10), 40);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day08/input.txt", 1000), 115885);
    }

    // #[test]
    // fn part2_test_input() {
    //     assert_eq!(part2("src/inputs/day08/test-input.txt"), 25272);
    // }

    // #[test]
    // fn part2_real() {
    //     assert_eq!(part2("src/inputs/day08/input.txt"), 11744693538946);
    // }
}
