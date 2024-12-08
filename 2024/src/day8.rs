use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BTreeMap, BTreeSet};

type Position = (i32, i32);
type Positions = BTreeMap<char, Vec<Position>>;

struct Grid {
    size: i32,
    antennas: Vec<Vec<Position>>,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Grid {
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
    Grid {
        size,
        antennas: positions.into_values().collect(),
    }
}

#[aoc(day8, part1)]
fn part1(input: &Grid) -> usize {
    let mut antinodes: BTreeSet<Position> = BTreeSet::new();
    let antennas = &input.antennas;
    let size = input.size;

    for positions in antennas.iter() {
        for antenna1 in 0..positions.len() {
            for antenna2 in antenna1 + 1..positions.len() {
                let (position1, position2) = (&positions[antenna1], &positions[antenna2]);
                if let Some(antinode) = get_antinode_n(-1, position1, position2, size) {
                    antinodes.insert(antinode);
                }
                if let Some(antinode) = get_antinode_n(-1, position2, position1, size) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}

#[aoc(day8, part2)]
fn part2(input: &Grid) -> usize {
    let mut antinodes: BTreeSet<Position> = BTreeSet::new();
    let antennas = &input.antennas;
    let size = input.size;

    for positions in antennas.iter() {
        for antenna1 in 0..positions.len() {
            for antenna2 in antenna1 + 1..positions.len() {
                let (position1, position2) = (&positions[antenna1], &positions[antenna2]);
                let mut forward = 0;
                while let Some(antinode) = get_antinode_n(forward, position1, position2, size) {
                    antinodes.insert(antinode);
                    forward += 1;
                }
                forward = -1;
                while let Some(antinode) = get_antinode_n(forward, position1, position2, size) {
                    antinodes.insert(antinode);
                    forward -= 1;
                }
            }
        }
    }

    antinodes.len()
}

fn get_antinode_n(n: i32, a: &Position, b: &Position, grid_size: i32) -> Option<Position> {
    let nx = a.0 * (1 - n) + n * b.0;
    let ny = a.1 * (1 - n) + n * b.1;

    if nx >= 0 && nx < grid_size && ny >= 0 && ny < grid_size {
        return Some((nx, ny));
    }

    None
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
        assert_eq!(part2(&parse(EXAMPLE)), 34);
    }
}
