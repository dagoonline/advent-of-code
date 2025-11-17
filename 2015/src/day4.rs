use aoc_runner_derive::aoc;
use md5::compute;

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let mut counter = 0;

    loop {
        let next = input.to_string() + &counter.to_string();
        let hash = compute(&next);
        if hash[0..2] == [0, 0] && hash[2] <= 0x10 {
            // println!("Test: {} Hash: {:x}", next, hash);
            return counter;
        }
        counter += 1;
    }
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let mut counter = 0;

    loop {
        let next = input.to_string() + &counter.to_string();
        let hash = compute(&next);
        if hash[0..3] == [0, 0, 0] {
            return counter;
        }
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
