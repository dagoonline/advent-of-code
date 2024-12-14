use core::fmt;
use std::{
    fmt::Display,
    ops::{AddAssign, Range},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Position(i32, i32);
impl AddAssign<&Velocity> for Position {
    fn add_assign(&mut self, velocity: &Velocity) {
        self.0 += velocity.0;
        self.1 += velocity.1;
    }
}

#[derive(Debug, Clone)]
struct Velocity(i32, i32);

#[derive(Debug, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn new(position: Position, velocity: Velocity) -> Robot {
        Robot { position, velocity }
    }

    fn step(&mut self) {
        self.position += &self.velocity;
    }

    fn teleport(&mut self, space_width: u32, space_height: u32) {
        self.position.0 = self.position.0.rem_euclid(space_width as i32);
        self.position.1 = self.position.1.rem_euclid(space_height as i32);
    }
}

#[derive(Debug)]
struct Space {
    width: u32,
    height: u32,
    robots: Vec<Robot>,
}

impl Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let positions = self
            .robots
            .iter()
            .map(|robot| (robot.position.0, robot.position.1))
            .collect_vec();
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                if positions.contains(&(x, y)) {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f)?
        }
        writeln!(f, "{}", "=".repeat(self.width as usize))
    }
}

impl Space {
    fn new(size_x: u32, size_y: u32) -> Self {
        Self {
            width: size_x,
            height: size_y,
            robots: vec![],
        }
    }

    fn add_robot(&mut self, robot: &Robot) {
        self.robots.push(robot.clone())
    }

    fn step(&mut self) {
        self.robots.iter_mut().for_each(|robot| {
            robot.step();
            robot.teleport(self.width, self.height);
        });
    }

    fn count_by_quadrant(&mut self) -> [u32; 4] {
        let mut quadrants_count = [0, 0, 0, 0];
        let quadrants: [(Range<i32>, Range<i32>); 4] = [
            (
                0..(self.width as i32 + 1) / 2 - 1,
                0..(self.height as i32 + 1) / 2 - 1,
            ),
            (
                (self.width as i32) / 2 + 1..self.width as i32,
                0..(self.height as i32) / 2,
            ),
            (
                0..(self.width as i32 + 1) / 2 - 1,
                (self.height as i32) / 2 + 1..self.height as i32,
            ),
            (
                (self.width as i32) / 2 + 1..self.width as i32,
                (self.height as i32) / 2 + 1..self.height as i32,
            ),
        ];

        self.robots
            .iter()
            .filter_map(|robot| {
                quadrants
                    .iter()
                    .enumerate()
                    .find(|(_, (range_x, range_y))| {
                        range_x.contains(&robot.position.0) && range_y.contains(&robot.position.1)
                    })
                    .map(|(quadrant, _)| quadrant)
            })
            .for_each(|quadrant| quadrants_count[quadrant] += 1);

        quadrants_count
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];
    // p=0,4 v=3,-3
    input.lines().for_each(|line| {
        let (position, velocity) = line.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
        let (position_x, position_y) = position.split_once(',').unwrap();
        let (velocity_x, velocity_y) = velocity.split_once(',').unwrap();

        robots.push(Robot::new(
            Position(position_x.parse().unwrap(), position_y.parse().unwrap()),
            Velocity(velocity_x.parse().unwrap(), velocity_y.parse().unwrap()),
        ))
    });

    robots
}

fn simulation(
    robots: &[Robot],
    space_width: u32,
    space_height: u32,
    steps: u32,
    display: bool,
) -> u32 {
    let mut space = Space::new(space_width, space_height);

    for robot in robots {
        space.add_robot(robot);
    }

    for i in 0..steps {
        space.step();
        if display {
            print!("iteration: {}\n{}", i + 1, space);
        }
    }

    space.count_by_quadrant().into_iter().product()
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> u32 {
    simulation(input, 101, 103, 100, false)
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> u32 {
    simulation(input, 101, 103, 10000, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(simulation(&parse(EXAMPLE), 11, 7, 100, false), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(simulation(&parse(EXAMPLE), 11, 7, 100, true), 12);
    }
}
