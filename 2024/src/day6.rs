use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::BTreeSet;

type Position2D = (i16, i16);
type Obstacles = BTreeSet<Position2D>;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    parse(input).run().get_history().len()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let original_world = parse(input);

    original_world
        .clone()
        .run()
        .get_history()
        .into_iter()
        .filter(|&position| {
            !original_world.is_obstacle(position) && original_world.get_guard_position() != position
        })
        .fold(0, |acc, position| {
            let mut world = original_world.clone();
            world.add_obstacle(position);
            match world.run().status {
                Status::Loop => acc + 1,
                _ => acc,
            }
        })
}

fn parse(input: &str) -> Gallivant {
    let mut obstacles: Obstacles = BTreeSet::new();
    let mut guard = Guard::new((0, 0), '<');
    let mut size = 0;

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            size += 1;
            line.chars()
                .enumerate()
                .map(|(x, item)| ((x as i16, y as i16), item))
                .collect_vec()
        })
        .for_each(|(pos, char)| match char {
            '#' => {
                obstacles.insert(pos);
            }
            '.' => (),
            g => guard = Guard::new(pos, g),
        });

    Gallivant::new(guard, obstacles, size)
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
enum Status {
    NotRun,
    Loop,
    Exit,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            _ => Direction::West,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Guard {
    pos: Position2D,
    facing: Direction,
}

impl Guard {
    fn new(pos: Position2D, char: char) -> Self {
        Self {
            pos,
            facing: char.into(),
        }
    }
}

#[derive(Clone)]
struct Gallivant {
    guard: Guard,
    obstacles: Obstacles,
    status: Status,
    steps: u32,
    size: i16,
    previous_points: BTreeSet<Guard>,
}

impl Gallivant {
    fn new(guard: Guard, obstacles: Obstacles, size: i16) -> Self {
        let mut previous_points = BTreeSet::new();
        previous_points.insert(guard);
        Self {
            guard,
            obstacles,
            size,
            previous_points,
            status: Status::NotRun,
            steps: 0,
        }
    }

    fn add_obstacle(&mut self, obstacle: Position2D) {
        self.obstacles.insert(obstacle);
    }

    fn is_obstacle(&self, obstacle: Position2D) -> bool {
        self.obstacles.contains(&obstacle)
    }

    fn next_position(&self) -> Position2D {
        let mut next_position = self.guard.pos;

        match self.guard.facing {
            Direction::North => next_position.1 -= 1,
            Direction::East => next_position.0 += 1,
            Direction::South => next_position.1 += 1,
            Direction::West => next_position.0 -= 1,
        }
        next_position
    }

    fn step(&mut self) {
        let next_position = self.next_position();

        if self.obstacles.contains(&next_position) {
            self.turn();
            return;
        }

        self.previous_points.insert(self.guard);
        self.guard.pos = next_position;
        self.steps += 1;
    }

    fn turn(&mut self) {
        use Direction::*;

        match self.guard.facing {
            North => self.guard.facing = East,
            East => self.guard.facing = South,
            South => self.guard.facing = West,
            West => self.guard.facing = North,
        }
    }

    fn is_within_limits(&self) -> bool {
        self.guard.pos.0 >= 0
            && self.guard.pos.0 < self.size
            && self.guard.pos.1 >= 0
            && self.guard.pos.1 < self.size
    }

    fn get_guard_position(&self) -> Position2D {
        self.guard.pos
    }

    fn was_already_here(&self) -> bool {
        self.previous_points.contains(&(self.guard))
    }

    fn get_history(&self) -> BTreeSet<Position2D> {
        let mut history = BTreeSet::new();
        self.previous_points.iter().for_each(|guard| {
            history.insert(guard.pos);
        });
        history
    }

    fn run(mut self) -> Self {
        loop {
            self.step();
            if self.was_already_here() {
                self.status = Status::Loop;
                break;
            }
            if !self.is_within_limits() {
                self.status = Status::Exit;
                break;
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
