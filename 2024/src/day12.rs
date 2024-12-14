use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::BTreeSet;

const BORDER: char = '#';

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position(i16, i16);

impl Position {
    fn add(&self, b: &Position) -> Position {
        Position(self.0 + b.0, self.1 + b.1)
    }

    fn add_edge(&self, edge: &EdgeCoordinates) -> EdgeCoordinates {
        (self.add(&edge.0), self.add(&edge.1), self.add(&edge.2))
    }

    fn cross() -> [Position; 4] {
        [
            Position(-1, 0),
            Position(1, 0),
            Position(0, -1),
            Position(0, 1),
        ]
    }
    fn clockwise() -> [Edge; 4] {
        [
            Edge::TopRight,
            Edge::BottomRight,
            Edge::BottomLeft,
            Edge::TopLeft,
        ]
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Position {
        Position(x as i16, y as i16)
    }
}

#[derive(PartialEq)]
enum Angle {
    Inner,
    Outer,
    None,
}

type EdgeCoordinates = (Position, Position, Position);
enum Edge {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

impl From<Edge> for EdgeCoordinates {
    fn from(ec: Edge) -> EdgeCoordinates {
        match ec {
            Edge::TopRight => (Position(0, -1), Position(1, -1), Position(1, 0)),
            Edge::BottomRight => (Position(1, 0), Position(1, 1), Position(0, 1)),
            Edge::BottomLeft => (Position(0, 1), Position(-1, 1), Position(-1, 0)),
            Edge::TopLeft => (Position(-1, 0), Position(-1, -1), Position(0, -1)),
        }
    }
}

#[derive(Debug)]
struct Region {
    positions: BTreeSet<Position>,
    area: i64,
}

impl Region {
    fn new(position: Position) -> Self {
        let mut positions = BTreeSet::new();
        positions.insert(position);

        Self { positions, area: 1 }
    }

    fn insert(&mut self, position: Position) {
        if self.positions.contains(&position) {
            return;
        }

        self.area += 1;
        self.positions.insert(position);
    }

    fn area(&self) -> i64 {
        self.area
    }

    fn neighbour_count(&self, position: &Position) -> i64 {
        Position::cross()
            .into_iter()
            .filter(|cross| self.positions.contains(&position.add(cross)))
            .count() as i64
    }

    fn compute_sides(&self, position: &Position, edge: &EdgeCoordinates) -> Angle {
        let absolute = position.add_edge(edge);
        if !self.positions.contains(&absolute.0) && !self.positions.contains(&absolute.2) {
            return Angle::Outer;
        }

        if self.positions.contains(&absolute.0)
            && !self.positions.contains(&absolute.1)
            && self.positions.contains(&absolute.2)
        {
            return Angle::Inner;
        }

        Angle::None
    }

    fn perimeter(&self) -> i64 {
        self.positions
            .iter()
            .map(|position| 4 - self.neighbour_count(position))
            .sum()
    }

    fn sides(&self) -> i64 {
        self.positions
            .iter()
            .map(|position| {
                Position::clockwise()
                    .into_iter()
                    .map(|edge| self.compute_sides(position, &edge.into()))
                    .filter(|angle| *angle != Angle::None)
                    .count() as i64
            })
            .sum()
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
        let mut region = Region::new((x, y).into());
        let mut same_region = Vec::new();
        same_region.push((x, y));

        while let Some((x, y)) = same_region.pop() {
            if !visited.contains(&(x, y)) {
                visited.insert((x, y));

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x.saturating_add_signed(dx as isize);
                    let ny = y.saturating_add_signed(dy as isize);

                    if board[ny][nx] == char {
                        region.insert((nx, ny).into());
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
        .map(|region| region.area() * region.perimeter())
        .sum()
}

#[aoc(day12, part2)]
fn part2(regions: &[Region]) -> i64 {
    regions
        .iter()
        .map(|region| region.area() * region.sides())
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

    const EXAMPLE4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const EXAMPLE5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 140);
        assert_eq!(part1(&parse(EXAMPLE2)), 772);
        assert_eq!(part1(&parse(EXAMPLE3)), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 80);
        assert_eq!(part2(&parse(EXAMPLE4)), 236);
        assert_eq!(part2(&parse(EXAMPLE5)), 368);
    }
}
