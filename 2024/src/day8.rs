use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BTreeMap, BTreeSet};

type Position = (i32, i32);
type Positions = BTreeMap<char, Vec<Position>>;

#[aoc_generator(day8)]
fn parse(input: &str) -> (Positions, i32) {
    let mut positions: Positions = BTreeMap::new();
    let mut size = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        size += 1;
        line.chars().enumerate().for_each(|(x, char)| {
            if char != '.' {
                positions
                    .entry(char)
                    .and_modify(|entries| entries.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        })
    });
    (positions, size)
}

#[aoc(day8, part1)]
fn part1(input: &(Positions, i32)) -> usize {
    let mut antinodes: BTreeSet<Position> = BTreeSet::new();
    let antennas = &input.0;
    let size = input.1;

    for (_antenna, positions) in antennas.iter() {
        for position1 in positions.iter() {
            for position2 in positions.iter() {
                if position1 != position2 {
                    antinodes.append(&mut get_antinodes(position1, position2, size));
                }
            }
        }
    }

    antinodes.len()
}

fn get_antinodes(a: &Position, b: &Position, size: i32) -> BTreeSet<Position> {
    let mut antinodes = BTreeSet::new();
    let x_diff = i32::abs_diff(a.0, b.0) as i32;
    let y_diff = i32::abs_diff(a.1, b.1) as i32;

    let (antinode1, antinode2): (Position, Position);
    if a.0 < b.0 {
        if a.1 < b.1 {
            antinode1 = (a.0 - x_diff, a.1 - y_diff);
            antinode2 = (b.0 + x_diff, b.1 + y_diff);
        } else {
            antinode1 = (a.0 - x_diff, a.1 + y_diff);
            antinode2 = (b.0 + x_diff, b.1 - y_diff);
        }
    } else if a.1 < b.1 {
        antinode1 = (a.0 + x_diff, a.1 - y_diff);
        antinode2 = (b.0 - x_diff, b.1 + y_diff);
    } else {
        antinode1 = (a.0 + x_diff, a.1 + y_diff);
        antinode2 = (b.0 - x_diff, b.1 - y_diff);
    }

    if antinode1.0 >= 0 && antinode1.0 < size && antinode1.1 >= 0 && antinode1.1 < size {
        antinodes.insert(antinode1);
    }
    if antinode2.0 >= 0 && antinode2.0 < size && antinode2.1 >= 0 && antinode2.1 < size {
        antinodes.insert(antinode2);
    }
    antinodes
}

#[aoc(day8, part2)]
fn part2(_input: &(Positions, i32)) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
