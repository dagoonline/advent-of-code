use itertools::Itertools;
use std::ops::AddAssign;

use aoc_runner_derive::{aoc, aoc_generator};
use foldhash::{HashMap, HashMapExt};

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        let target = parts
            .next()
            .unwrap()
            .chars()
            .filter(|c| *c != '[' && *c != ']')
            .map(|c| c == '#')
            .enumerate()
            .fold(0, |acc, (i, n)| acc | (n as u16) << i);
        let joltages = parts
            .next_back()
            .unwrap()
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|j| j.parse::<u16>().unwrap())
            .collect();
        let buttons = parts
            .take_while(|x| x.starts_with('('))
            .map(|b| {
                b[1..b.len() - 1]
                    .split(',')
                    .map(|c| c.parse::<u16>().unwrap())
                    .fold(0, |acc, n| acc | (1 << n))
            })
            .collect();

        machines.push(Machine {
            target,
            buttons,
            joltages,
        })
    }

    machines
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> u16 {
    input.iter().fold(0, |acc, machine| {
        combinations(machine)
            .get(&machine.target)
            .unwrap()
            .iter()
            .map(|v| v.len())
            .min()
            .unwrap() as u16
            + acc
    })
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> u64 {
    input.iter().fold(0, |acc, machine| {
        let min = get_min_presses(
            &mut HashMap::new(),
            &combinations(machine),
            &machine.joltages,
        )
        .unwrap() as u64;
        min + acc
    })
}

#[derive(Debug)]
struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

#[derive(Default, Clone, Copy)]
struct State {
    indicator: u16,
    steps: u16,
}

impl AddAssign<&u16> for State {
    fn add_assign(&mut self, rhs: &u16) {
        self.indicator ^= rhs;
        self.steps += 1;
    }
}

fn combinations(machine: &Machine) -> HashMap<u16, Vec<Vec<u16>>> {
    let mut presses: HashMap<u16, Vec<Vec<u16>>> = HashMap::new();

    for n_presses in 0..=machine.buttons.len() {
        for b in machine.buttons.iter().copied().combinations(n_presses) {
            let mut state = State::default();

            for button in b.iter() {
                state += button
            }

            presses
                .entry(state.indicator)
                .and_modify(|v| v.push(b.clone()))
                .or_insert(vec![b]);
        }
    }

    presses
}

// Instead using a boring linear equation solver, found a clever and part1-continuation solution at Reddit which I implemented below in Rust.
// "Bifurcate your way to victory!" from u/tenthmascot (https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/)
// Code based on excellent implementation in python from Josiah Winslow (https://aoc.winslowjosiah.com/solutions/2025/day/10/)
fn get_min_presses(
    cache: &mut HashMap<Vec<u16>, u16>,
    patterns: &HashMap<u16, Vec<Vec<u16>>>,
    target: &[u16],
) -> Option<u16> {
    if let Some(&cached) = cache.get(target) {
        return Some(cached);
    }

    if target.iter().all(|&x| x == 0) {
        return Some(0);
    }

    let indicator = target
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &val)| acc | (val % 2) << i);

    let press_patterns = patterns.get(&indicator)?;
    let mut result: Option<u16> = None;
    for pattern in press_patterns {
        let mut target_after: Vec<i32> = target.iter().map(|&c| c as i32).collect();

        for &button in pattern.iter() {
            let mut mask = button;
            let mut bit_pos = 0;
            while mask > 0 {
                if mask & 1 == 1 {
                    target_after[bit_pos] -= 1;
                }
                mask >>= 1;
                bit_pos += 1;
            }
        }

        if target_after.iter().any(|&x| x < 0) {
            continue;
        }

        // Divide by 2 and recurse
        let half_target: Vec<u16> = target_after.iter().map(|&v| (v / 2) as u16).collect();

        if let Some(num_half_presses) = get_min_presses(cache, patterns, &half_target) {
            let num_presses = pattern.len() as u16 + 2 * num_half_presses;
            result = Some(result.map_or(num_presses, |r| r.min(num_presses)));
        }
    }

    if let Some(res) = result {
        cache.insert(target.to_vec(), res);
    }
    result
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
        assert_eq!(part2(&parse(INPUT)), 33);
    }
}
