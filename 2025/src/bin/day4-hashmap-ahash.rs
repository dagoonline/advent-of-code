use ahash::{HashMap, HashMapExt};

fn parse(input: &str) -> HashMap<(usize, usize), u8> {
    let mut grid: HashMap<(usize, usize), u8> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '@') {
            grid.insert((x + 1, y + 1), 0);
        }
    }

    for (x, y) in grid.clone().into_keys() {
        let mut v = 0;

        v += grid.contains_key(&(x - 1, y - 1)) as u8;
        v += grid.contains_key(&(x, y - 1)) as u8;
        v += grid.contains_key(&(x + 1, y - 1)) as u8;
        v += grid.contains_key(&(x - 1, y)) as u8;
        v += grid.contains_key(&(x + 1, y)) as u8;
        v += grid.contains_key(&(x - 1, y + 1)) as u8;
        v += grid.contains_key(&(x, y + 1)) as u8;
        v += grid.contains_key(&(x + 1, y + 1)) as u8;

        grid.entry((x, y)).and_modify(|val| *val = v);
    }

    grid
}

fn get_acceseable(rolls: &HashMap<(usize, usize), u8>) -> u16 {
    rolls.iter().fold(0, |acc, (_, v)| acc + (v < &4) as u16)
}

fn step(rolls: &mut HashMap<(usize, usize), u8>) {
    let keys = rolls
        .iter()
        .filter(|(_, v)| **v < 4)
        .map(|((x, y), _)| (*x, *y))
        .collect::<Vec<(usize, usize)>>();

    for (x, y) in keys {
        rolls.remove(&(x, y));

        rolls.entry((x - 1, y - 1)).and_modify(|v| *v -= 1);
        rolls.entry((x, y - 1)).and_modify(|v| *v -= 1);
        rolls.entry((x + 1, y - 1)).and_modify(|v| *v -= 1);
        rolls.entry((x - 1, y)).and_modify(|v| *v -= 1);
        rolls.entry((x + 1, y)).and_modify(|v| *v -= 1);
        rolls.entry((x - 1, y + 1)).and_modify(|v| *v -= 1);
        rolls.entry((x, y + 1)).and_modify(|v| *v -= 1);
        rolls.entry((x + 1, y + 1)).and_modify(|v| *v -= 1);
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
