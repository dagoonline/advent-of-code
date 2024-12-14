use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug)]
struct Machine {
    x_a: i64,
    x_b: i64,
    y_a: i64,
    y_b: i64,
    x_prize: i64,
    y_prize: i64,
}

#[derive(Debug)]
struct ParseMachineError;

impl FromStr for Machine {
    type Err = ParseMachineError;

    //Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n
    fn from_str(input: &str) -> Result<Machine, ParseMachineError> {
        let mut parts = input.lines();
        let (x_a, y_a) = parts
            .next()
            .unwrap()
            .strip_prefix("Button A: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (x_b, y_b) = parts
            .next()
            .unwrap()
            .strip_prefix("Button B: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (x_prize, y_prize) = parts
            .next()
            .unwrap()
            .strip_prefix("Prize: X=")
            .unwrap()
            .split_once(", Y=")
            .unwrap();
        Ok(Machine {
            x_a: x_a.parse().unwrap(),
            x_b: x_b.parse().unwrap(),
            y_a: y_a.parse().unwrap(),
            y_b: y_b.parse().unwrap(),
            x_prize: x_prize.parse().unwrap(),
            y_prize: y_prize.parse().unwrap(),
        })
    }
}

impl Machine {
    fn calculate_tokens(&self, prize_fix: i64) -> i64 {
        let x_prize = self.x_prize + prize_fix;
        let y_prize = self.y_prize + prize_fix;

        let a_presses =
            (x_prize * self.y_b - self.x_b * y_prize) / (self.x_a * self.y_b - self.x_b * self.y_a);
        let b_presses =
            (x_prize * self.y_a - self.x_a * y_prize) / (self.x_b * self.y_a - self.x_a * self.y_b);

        if (
            self.x_a * a_presses + self.x_b * b_presses,
            self.y_a * a_presses + self.y_b * b_presses,
        ) == (x_prize, y_prize)
        {
            3 * a_presses + b_presses
        } else {
            0
        }
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|machine| machine.parse().unwrap())
        .collect::<Vec<Machine>>()
}

#[aoc(day13, part1)]
fn part1(input: &[Machine]) -> i64 {
    input
        .iter()
        .map(|machine| machine.calculate_tokens(0))
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Machine]) -> i64 {
    input
        .iter()
        .map(|machine| machine.calculate_tokens(10000000000000))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 480);
    }
}
