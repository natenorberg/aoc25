use core::panic;
use std::fs;

pub fn part1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let lines: Vec<&str> = input.lines().collect();
    get_total_splits(&lines)
}

pub fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Couln't read the file");
    let lines: Vec<&str> = input.lines().collect();
    get_total_timelines(&lines)
}

// Data Structures ============================================================
#[derive(Debug, PartialEq, Clone, Copy)]
struct Beam {
    index: usize,
    timelines: u64,
}

// Logic ======================================================================
fn get_total_splits(lines: &[&str]) -> u32 {
    let mut total_splits = 0;
    let start_beam = lines[0].find('S').unwrap();
    let mut beams = vec![start_beam];

    for line in &lines[1..lines.len() - 1] {
        let (next_beams, splits) = get_next_beams(&beams, line);
        beams = next_beams;
        total_splits += splits;
    }

    total_splits
}

fn get_next_beams(beams: &[usize], line: &str) -> (Vec<usize>, u32) {
    let mut next = Vec::new();
    let mut splits = 0;

    beams
        .iter()
        .for_each(|beam| match line.chars().nth(*beam).unwrap() {
            '.' => next.push(*beam),
            '^' => {
                if *beam > 0 {
                    next.push(*beam - 1);
                }
                if *beam < line.len() - 1 {
                    next.push(*beam + 1);
                }
                splits += 1;
            }
            _ => panic!("Unknown character"),
        });

    next.sort();
    next.dedup();
    (next, splits)
}

fn get_total_timelines(lines: &[&str]) -> u64 {
    let mut beams = vec![Beam {
        index: lines[0].find('S').unwrap(),
        timelines: 1,
    }];

    for line in &lines[1..lines.len() - 1] {
        let next_beams = get_next_beams_multiworld(&beams, line);
        beams = next_beams;
    }

    count_timelines(&beams)
}

fn get_next_beams_multiworld(beams: &[Beam], line: &str) -> Vec<Beam> {
    let mut next: Vec<Beam> = Vec::new();

    beams
        .iter()
        .for_each(|beam| match line.chars().nth(beam.index).unwrap() {
            '.' => next.push(*beam),
            '^' => {
                if beam.index > 0 {
                    next.push(Beam {
                        index: beam.index - 1,
                        timelines: beam.timelines,
                    });
                }
                if beam.index < line.len() - 1 {
                    next.push(Beam {
                        index: beam.index + 1,
                        timelines: beam.timelines,
                    });
                }
            }
            _ => panic!("Unknown character"),
        });

    collapse_timelines(&next)
}

fn collapse_timelines(beams: &[Beam]) -> Vec<Beam> {
    let mut indexes: Vec<usize> = beams.iter().map(|b| b.index).collect();
    indexes.dedup();

    indexes
        .iter()
        .map(|idx| {
            let timelines: u64 = beams
                .iter()
                .filter(|b| b.index == *idx)
                .map(|b| b.timelines)
                .sum();

            Beam {
                index: *idx,
                timelines,
            }
        })
        .collect()
}

fn count_timelines(beams: &[Beam]) -> u64 {
    beams.iter().map(|b| b.timelines).sum()
}

// Tests ======================================================================
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_next_beams() {
        let input =
            fs::read_to_string("src/inputs/day07/test-input.txt").expect("Couln't read the file");
        let lines: Vec<&str> = input.lines().collect();

        assert_eq!(get_next_beams(&[7], lines[1]), (vec![7], 0));
        assert_eq!(get_next_beams(&[7], lines[2]), (vec![6, 8], 1));
        assert_eq!(get_next_beams(&[6, 8], lines[3]), (vec![6, 8], 0));
        assert_eq!(get_next_beams(&[6, 8], lines[4]), (vec![5, 7, 9], 2));

        assert_eq!(get_next_beams(&[0], "^..."), (vec![1], 1));
        assert_eq!(get_next_beams(&[3], "...^"), (vec![2], 1));

        assert_eq!(get_next_beams(&[1], ".^.."), (vec![0, 2], 1));
        assert_eq!(get_next_beams(&[2], "..^."), (vec![1, 3], 1));
    }

    #[test]
    fn test_get_next_beams_multiworld() {
        let input =
            fs::read_to_string("src/inputs/day07/test-input.txt").expect("Couln't read the file");
        let lines: Vec<&str> = input.lines().collect();

        fn beam(index: usize, timelines: u64) -> Beam {
            Beam { index, timelines }
        }

        assert_eq!(
            get_next_beams_multiworld(&[beam(7, 1)], lines[1]),
            vec![beam(7, 1)]
        );
        assert_eq!(
            get_next_beams_multiworld(&[beam(7, 1)], lines[2]),
            vec![beam(6, 1), beam(8, 1)]
        );
        assert_eq!(
            get_next_beams_multiworld(&[beam(6, 1), beam(8, 1)], lines[3]),
            vec![beam(6, 1), beam(8, 1)]
        );
        assert_eq!(
            get_next_beams_multiworld(&[beam(6, 1), beam(8, 1)], lines[4]),
            vec![beam(5, 1), beam(7, 2), beam(9, 1)]
        );
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(part1("src/inputs/day07/test-input.txt"), 21);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1("src/inputs/day07/input.txt"), 1687);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2("src/inputs/day07/test-input.txt"), 40);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2("src/inputs/day07/input.txt"), 390684413472684);
    }
}
