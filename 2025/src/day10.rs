use std::{collections::VecDeque, ops::Add};

use aoc_runner_derive::{aoc, aoc_generator};
use foldhash::{HashSet, HashSetExt};

#[derive(Debug)]
struct Machine {
    target: u16,
    buttons: Vec<u16>,
}

#[derive(Default, Clone, Copy)]
struct State {
    indicator: u16,
    steps: u16,
}

impl Add<&u16> for State {
    type Output = State;

    fn add(mut self, rhs: &u16) -> Self::Output {
        self.indicator ^= rhs;
        self.steps += 1;

        self
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let mut buttons = vec![];

        let target = parts
            .next()
            .unwrap()
            .chars()
            .filter(|c| *c != '[' && *c != ']')
            .map(|c| c == '#')
            .enumerate()
            .fold(0, |acc, (i, n)| acc | (n as u16) << i);

        for b in parts.take_while(|x| x.starts_with('(')) {
            let button: u16 = b[1..b.len() - 1]
                .split(',')
                .map(|c| c.parse::<u16>().unwrap())
                .fold(0, |acc, n| acc | (1 << n));

            buttons.push(button)
        }

        machines.push(Machine { target, buttons })
    }

    machines
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> u16 {
    input.iter().fold(0, |acc, machine| {
        let mut found = HashSet::new();
        let mut pending = VecDeque::new();

        pending.push_front(State::default());

        while let Some(state) = pending.pop_back() {
            if state.indicator == machine.target {
                return acc + state.steps;
            }

            if found.contains(&state.indicator) {
                continue;
            }

            found.insert(state.indicator);

            machine
                .buttons
                .iter()
                .for_each(|button| pending.push_front(state + button));
        }

        panic!("indicator unreachable")
    })
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 7);
    }
}
