use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<u64>> {
    let mut batteries = vec![];

    for line in input.lines() {
        batteries.push(
            line.chars()
                .map(|c| u64::from(c.to_digit(10).unwrap()))
                .collect(),
        )
    }

    batteries
}

fn get_jolts(input: &[Vec<u64>], digits: usize) -> u64 {
    let mut jolts = 0;

    for battery in input {
        let mut next = 0;
        let mut battery_jolts = 0;

        for remaining in (0..digits).rev() {
            let n = battery[next..(battery.len() - remaining)]
                .iter()
                .max()
                .unwrap()
                .to_owned();

            next += battery[next..].iter().position(|&v| v == n).unwrap() + 1;
            battery_jolts *= 10;
            battery_jolts += n;
        }

        jolts += battery_jolts;
    }

    jolts
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u64>]) -> u64 {
    get_jolts(input, 2)
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u64>]) -> u64 {
    get_jolts(input, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 3121910778619);
    }
}
