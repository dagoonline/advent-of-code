use core::fmt;
use std::{fmt::Display, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    movable: bool,
}

impl Block {
    fn is_movable(&self) -> bool {
        self.movable
    }
}

#[derive(Debug, Clone)]
struct Map {
    robot: (u16, u16),
    blocks: Vec<Vec<Option<Block>>>,
}

impl Map {
    fn displace(&mut self, (x, y): (u16, u16), direction: Direction) -> bool {
        let (nx, ny) = match direction {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        if x >= self.blocks[0].len() as u16 || y >= self.blocks.len() as u16 {
            return false;
        }
        let block = self.blocks[ny as usize][nx as usize];

        if block.is_none() || (block.unwrap().is_movable() && self.displace((nx, ny), direction)) {
            self.blocks[ny as usize][nx as usize] = self.blocks[y as usize][x as usize];
            self.blocks[y as usize][x as usize] = None;
            return true;
        }
        false
    }

    fn get_robot(&self) -> (u16, u16) {
        self.robot
    }

    fn move_robot(&mut self, direction: Direction) {
        let (mut x, mut y) = self.robot;
        match direction {
            Direction::Up => y -= 1,
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
        };
        self.robot = (x, y);
    }

    fn get_boxes(&self) -> Vec<(u16, u16)> {
        self.blocks
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, block)| block.is_some() && block.unwrap().is_movable())
                    .map(move |(x, _)| (x as u16, y as u16))
            })
            .collect_vec()
    }
}

struct MapError;

impl FromStr for Map {
    type Err = MapError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut blocks = vec![];
        let mut robot = (0, 0);
        for line_vec in input.lines().enumerate().map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| (x, y, char))
                .collect_vec()
        }) {
            let mut line = vec![];

            for (x, y, char) in line_vec {
                match char {
                    '#' => line.push(Some(Block { movable: false })),
                    '.' => line.push(None),
                    'O' => line.push(Some(Block { movable: true })),
                    '@' => {
                        line.push(None);
                        robot = (x as u16, y as u16)
                    }
                    _ => return Err(MapError),
                }
            }
            blocks.push(line);
        }

        Ok(Map { robot, blocks })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.blocks.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if c.is_none() {
                    if (x as u16, y as u16) == self.get_robot() {
                        write!(f, "@")?
                    } else {
                        write!(f, ".")?
                    }
                } else if c.unwrap().is_movable() {
                    write!(f, "O")?
                } else {
                    write!(f, "#")?
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}
#[aoc_generator(day15)]
fn parse(input: &str) -> (Map, String) {
    let (map, path) = input.split_once("\n\n").unwrap();
    let Ok(map) = map.parse() else {
        panic!("Wrong map")
    };

    (map, path.to_string())
}

#[aoc(day15, part1)]
fn part1((map, path): &(Map, String)) -> i64 {
    let mut map = map.clone();
    for char in path.chars() {
        match char {
            '^' => {
                if map.displace(map.get_robot(), Direction::Up) {
                    map.move_robot(Direction::Up);
                }
            }
            'v' => {
                if map.displace(map.get_robot(), Direction::Down) {
                    map.move_robot(Direction::Down);
                }
            }
            '>' => {
                if map.displace(map.get_robot(), Direction::Right) {
                    map.move_robot(Direction::Right);
                }
            }
            '<' => {
                if map.displace(map.get_robot(), Direction::Left) {
                    map.move_robot(Direction::Left);
                }
            }
            '\n' => (),
            _ => unreachable!(),
        };
    }

    map.get_boxes()
        .iter()
        .fold(0, |acc, &(x, y)| acc + y as i64 * 100 + x as i64)
}

// #[aoc(day15, part2)]
// fn part2(input: &str) -> i64 {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const EXAMPLE2: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE1)), 10092);
        assert_eq!(part1(&parse(EXAMPLE2)), 2028);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
