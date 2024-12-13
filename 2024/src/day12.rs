use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::BTreeSet;

const BORDER: char = '#';

#[derive(Debug)]
struct Region {
    positions: BTreeSet<(i16, i16)>,
    area: i64,
    perimeter: i64,
    sides: i64,
}

impl Region {
    fn new(position: (i16, i16)) -> Self {
        let mut positions = BTreeSet::new();
        positions.insert(position);

        Self {
            positions,
            area: 1,
            perimeter: 4,
            sides: 4,
        }
    }

    fn insert(&mut self, (x, y): (i16, i16)) {
        if self.positions.contains(&(x, y)) {
            return;
        }

        self.area += 1;
        let mut xneighbours = 0;
        for dx in [-1, 1] {
            if self.positions.contains(&(x + dx, y)) {
                xneighbours += 1;
            }
        }
        let mut yneighbours = 0;
        for dy in [-1, 1] {
            if self.positions.contains(&(x, y + dy)) {
                yneighbours += 1;
            }
        }
        let (add_perimeter, add_sides) = match (xneighbours, yneighbours) {
            (0, 0) => (4, 4),
            (0, 1) | (1, 0) => (2, 0),
            (0, 2) | (2, 0) => (0, -2),
            (1, 1) => (0, -2),
            (2, 1) | (1, 2) => (-2, -2),
            (2, 2) => (-4, -4),
            _ => unreachable!(),
        };
        self.positions.insert((x, y));
        self.perimeter += add_perimeter;
        self.sides += add_sides;
    }

    fn get_area(&self) -> i64 {
        self.area
    }

    fn get_perimeter(&self) -> i64 {
        self.perimeter
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Region> {
    let mut board = vec![vec![]];
    board.append(
        &mut input
            .lines()
            .map(|line| {
                let mut l = vec![BORDER];
                l.append(&mut line.chars().collect_vec());
                l.push(BORDER);
                l
            })
            .collect_vec(),
    );

    let size = board[1].len();
    let border = std::iter::repeat(BORDER);
    board[0] = border.clone().take(size).collect();
    board.push(border.take(size).collect());

    let mut regions = Vec::new();
    let mut to_visit = BTreeSet::new();
    let mut visited = BTreeSet::new();
    to_visit.insert((1, 1));

    while let Some((x, y)) = to_visit.iter().next().cloned() {
        to_visit.remove(&(x, y));
        if visited.contains(&(x, y)) {
            continue;
        }

        let char = board[y][x];
        let mut region = Region::new((x as i16, y as i16));
        let mut same_region = Vec::new();
        same_region.push((x, y));

        while let Some((x, y)) = same_region.pop() {
            if !visited.contains(&(x, y)) {
                visited.insert((x, y));

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x.saturating_add_signed(dx as isize);
                    let ny = y.saturating_add_signed(dy as isize);

                    if board[ny][nx] == char {
                        region.insert((nx as i16, ny as i16));
                        same_region.push((nx, ny));
                    } else if board[ny][nx] != BORDER {
                        to_visit.insert((nx, ny));
                    }
                }
            }
        }
        regions.push(region);
    }

    regions
}

#[aoc(day12, part1)]
fn part1(regions: &[Region]) -> i64 {
    regions
        .iter()
        .map(|region| region.get_area() * region.get_perimeter())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC
";

    const EXAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 140);
        assert_eq!(part1(&parse(EXAMPLE2)), 772);
        assert_eq!(part1(&parse(EXAMPLE3)), 1930);
    }
}
