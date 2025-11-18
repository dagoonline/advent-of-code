use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::TurnOn => "turn on",
            Self::TurnOff => "turn off",
            Self::Toggle => "switch",
        };
        write!(f, "{s}")
    }
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            _ => Action::Toggle,
        }
    }
}

type Coordinate = (u32, u32);

struct Instruction {
    action: Action,
    start: Coordinate,
    end: Coordinate,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - ({},{}) to ({},{})",
            self.action, self.start.0, self.start.1, self.end.0, self.end.1,
        )
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Instruction> {
    let re = Regex::new(
        r"(?P<action>turn off|turn on|toggle) (?P<startx>[\d]+),(?P<starty>[\d]+) through (?P<endx>[\d]+),(?P<endy>[\d]+)",
    )
    .unwrap();

    let mut actions = vec![];
    for line in input.lines() {
        let parse = re.captures(line).unwrap();

        let action = Instruction {
            action: parse["action"].into(),
            start: (
                parse["startx"].parse().unwrap(),
                parse["starty"].parse().unwrap(),
            ),
            end: (
                parse["endx"].parse().unwrap(),
                parse["endy"].parse().unwrap(),
            ),
        };
        actions.push(action);
    }

    actions
}

#[aoc(day6, part1)]
fn part1(input: &[Instruction]) -> u32 {
    let mut grid = HashSet::new();
    for i in input {
        match i.action {
            Action::TurnOn => {
                for x in i.start.0..i.end.0 + 1 {
                    for y in i.start.1..i.end.1 + 1 {
                        grid.insert((x, y));
                    }
                }
            }
            Action::TurnOff => {
                for x in i.start.0..i.end.0 + 1 {
                    for y in i.start.1..i.end.1 + 1 {
                        grid.remove(&(x, y));
                    }
                }
            }
            Action::Toggle => {
                for x in i.start.0..i.end.0 + 1 {
                    for y in i.start.1..i.end.1 + 1 {
                        let point = (x, y);
                        if grid.contains(&point) {
                            grid.remove(&point);
                        } else {
                            grid.insert(point);
                        }
                    }
                }
            }
        }
    }

    grid.len() as u32
}

#[aoc(day6, part2)]
fn part2(input: &[Instruction]) -> u32 {
    let mut grid = HashMap::new();
    for i in input {
        match i.action {
            Action::TurnOn => {
                for x in i.start.0..i.end.0 + 1 {
                    for y in i.start.1..i.end.1 + 1 {
                        grid.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
                    }
                }
            }
            Action::TurnOff => {
                for x in i.start.0..i.end.0 + 1 {
                    for y in i.start.1..i.end.1 + 1 {
                        grid.entry((x, y))
                            .and_modify(|v| {
                                if *v > 0 {
                                    *v -= 1
                                }
                            })
                            .or_insert(0);
                    }
                }
            }
            Action::Toggle => {
                for x in i.start.0..i.end.0 + 1 {
                    for y in i.start.1..i.end.1 + 1 {
                        grid.entry((x, y)).and_modify(|v| *v += 2).or_insert(2);
                    }
                }
            }
        }
    }

    grid.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500")), 998996);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "turn on 0,0 through 0,0\ntoggle 0,0 through 999,999"
            )),
            2000001
        );
    }
}
