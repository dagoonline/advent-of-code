use nohash::{IsEnabled, NoHashHasher};
use std::{collections::HashMap, hash::BuildHasherDefault};

#[derive(PartialEq, Eq, Clone)]
struct Key((usize, usize));

impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_usize(self.0.1);
        hasher.write_usize(self.0.1)
    }
}

impl IsEnabled for Key {}

type Data = HashMap<Key, u8, BuildHasherDefault<NoHashHasher<Key>>>;

fn parse(input: &str) -> Data {
    let mut grid: Data = HashMap::with_hasher(BuildHasherDefault::default());

    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '@') {
            grid.insert(Key((x + 1, y + 1)), 0);
        }
    }

    for Key((x, y)) in grid.clone().into_keys() {
        let mut v = 0;

        v += grid.contains_key(&Key((x - 1, y - 1))) as u8;
        v += grid.contains_key(&Key((x, y - 1))) as u8;
        v += grid.contains_key(&Key((x + 1, y - 1))) as u8;
        v += grid.contains_key(&Key((x - 1, y))) as u8;
        v += grid.contains_key(&Key((x + 1, y))) as u8;
        v += grid.contains_key(&Key((x - 1, y + 1))) as u8;
        v += grid.contains_key(&Key((x, y + 1))) as u8;
        v += grid.contains_key(&Key((x + 1, y + 1))) as u8;

        grid.entry(Key((x, y))).and_modify(|val| *val = v);
    }

    grid
}

fn get_acceseable(rolls: &Data) -> u16 {
    rolls.iter().fold(0, |acc, (_, v)| acc + (v < &4) as u16)
}

fn step(rolls: &mut Data) {
    let keys = rolls
        .iter()
        .filter(|(_, v)| **v < 4)
        .map(|(Key((x, y)), _)| (*x, *y))
        .collect::<Vec<(usize, usize)>>();

    for (x, y) in keys {
        rolls.remove(&Key((x, y)));

        rolls.entry(Key((x - 1, y - 1))).and_modify(|v| *v -= 1);
        rolls.entry(Key((x, y - 1))).and_modify(|v| *v -= 1);
        rolls.entry(Key((x + 1, y - 1))).and_modify(|v| *v -= 1);
        rolls.entry(Key((x - 1, y))).and_modify(|v| *v -= 1);
        rolls.entry(Key((x + 1, y))).and_modify(|v| *v -= 1);
        rolls.entry(Key((x - 1, y + 1))).and_modify(|v| *v -= 1);
        rolls.entry(Key((x, y + 1))).and_modify(|v| *v -= 1);
        rolls.entry(Key((x + 1, y + 1))).and_modify(|v| *v -= 1);
    }
}

fn part1(input: &str) -> u16 {
    get_acceseable(&parse(input))
}

fn part2(input: &str) -> u16 {
    let mut grid = parse(input);
    let mut acc = 0;

    while let next = get_acceseable(&grid)
        && next != 0
    {
        acc += next;

        step(&mut grid);
    }

    acc
}

fn main() {
    let s = include_str!("../../input/2025/day4.txt");

    println!("part 1: {}", part1(s));
    println!("part 2: {}", part2(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 43);
    }
}
