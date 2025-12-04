use tinyset::{Fits64, Set64};

#[derive(Copy, Clone)]
struct Roll(u8, u8);

impl Fits64 for Roll {
    unsafe fn from_u64(x: u64) -> Self {
        let c1 = ((x & 0xff00) >> 8) as u8;
        let c2 = (x & 0x00ff) as u8;

        Roll(c1, c2)
    }

    fn to_u64(self) -> u64 {
        (self.0 as u64) << 8 | self.1 as u64
    }
}

type Data = Set64<Roll>;

fn parse(input: &str) -> Data {
    let mut grid: Data = Data::new();

    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '@') {
            grid.insert(Roll((x + 1) as u8, (y + 1) as u8));
        }
    }

    grid
}

fn get_acceseable(rolls: &Data) -> u16 {
    let mut total = 0;

    for Roll(x, y) in rolls.iter() {
        let mut v = 0;

        v += rolls.contains(Roll(x - 1, y - 1)) as u8;
        v += rolls.contains(Roll(x, y - 1)) as u8;
        v += rolls.contains(Roll(x + 1, y - 1)) as u8;
        v += rolls.contains(Roll(x - 1, y)) as u8;
        v += rolls.contains(Roll(x + 1, y)) as u8;
        v += rolls.contains(Roll(x - 1, y + 1)) as u8;
        v += rolls.contains(Roll(x, y + 1)) as u8;
        v += rolls.contains(Roll(x + 1, y + 1)) as u8;

        if v < 4 {
            total += 1
        }
    }

    total
}

fn step(rolls: &mut Data) {
    let mut delete = vec![];

    for Roll(x, y) in rolls.iter() {
        let mut v = 0;
        v += rolls.contains(Roll(x - 1, y - 1)) as u8;
        v += rolls.contains(Roll(x, y - 1)) as u8;
        v += rolls.contains(Roll(x + 1, y - 1)) as u8;
        v += rolls.contains(Roll(x - 1, y)) as u8;
        v += rolls.contains(Roll(x + 1, y)) as u8;
        v += rolls.contains(Roll(x - 1, y + 1)) as u8;
        v += rolls.contains(Roll(x, y + 1)) as u8;
        v += rolls.contains(Roll(x + 1, y + 1)) as u8;

        if v < 4 {
            delete.push((x, y));
        }
    }

    for (x, y) in delete.into_iter() {
        rolls.remove(&Roll(x, y));
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
