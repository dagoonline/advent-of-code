use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

#[aoc(day11, part1)]
fn part1(stones: &[u64]) -> u64 {
    blink_n(25, stones)
}

#[aoc(day11, part2)]
fn part2(stones: &[u64]) -> u64 {
    blink_n(75, stones)
}

fn blink(n: u16, stone: u64, cache: &mut HashMap<(u64, u16), u64>) -> u64 {
    if n == 0 {
        return 1;
    }

    if cache.contains_key(&(stone, n)) {
        return *cache.get(&(stone, n)).unwrap();
    }

    let next = if stone == 0 {
        blink(n - 1, 1, cache)
    } else if stone.to_string().len() % 2 == 0 {
        let stone_str = stone.to_string();
        let half = stone_str.len() / 2;
        let left = stone_str[..half].parse::<u64>().unwrap();
        let right = stone_str[half..].parse::<u64>().unwrap();
        blink(n - 1, left, cache) + blink(n - 1, right, cache)
    } else {
        blink(n - 1, stone * 2024, cache)
    };
    cache.insert((stone, n), next);
    next
}

fn blink_n(blinks: u16, stones: &[u64]) -> u64 {
    let cache = &mut HashMap::new();

    stones
        .iter()
        .map(|&stone| blink(blinks, stone, cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_2_example() {
        assert_eq!(blink_n(6, &parse("125 17")), 22);
        assert_eq!(blink_n(25, &parse("125 17")), 55312);
    }
}
