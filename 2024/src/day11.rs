use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

#[aoc(day11, part1)]
fn part1(stones: &Vec<u64>) -> u64 {
    blink_n(25, stones)
}

#[aoc(day11, part2)]
fn part2(stones: &Vec<u64>) -> u64 {
    blink_n(75, stones)
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();
    for stone in stones {
        if *stone == 0 {
            new_stones.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            let stone_str = stone.to_string();
            let half = stone_str.len() / 2;
            let left = stone_str[..half].parse::<u64>().unwrap();
            let right = stone_str[half..].parse::<u64>().unwrap();
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}

fn blink_n(blinks: u64, stones: &Vec<u64>) -> u64 {
    let mut stones = stones.clone();

    for _ in 0..blinks {
        stones = blink(&stones);
    }

    stones.len() as u64
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
