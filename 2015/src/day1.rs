use std::fmt::{self, Display};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i16> {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            _ => -1,
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i16]) -> i16 {
    input.iter().sum()
}

#[derive(PartialEq, Debug)]
enum Solution {
    Found(usize),
    NotFound,
}

impl Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Solution::Found(solution) => write!(f, "Solution: {solution}"),
            Solution::NotFound => write!(f, "Solution: Not found"),
        }
    }
}

#[aoc(day1, part2)]
fn part2(input: &[i16]) -> Solution {
    let mut floor = 0;

    for (index, &movement) in input.iter().enumerate() {
        floor += movement;
        if floor == -1 {
            return Solution::Found(index + 1);
        }
    }
    Solution::NotFound
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("(())")), 0);
        assert_eq!(part1(&parse("()()")), 0);
        assert_eq!(part1(&parse("(((")), 3);
        assert_eq!(part1(&parse("(()(()(")), 3);
        assert_eq!(part1(&parse("))(((((")), 3);
        assert_eq!(part1(&parse("())")), -1);
        assert_eq!(part1(&parse("))(")), -1);
        assert_eq!(part1(&parse(")))")), -3);
        assert_eq!(part1(&parse(")())())")), -3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(")")), Solution::Found(1));
        assert_eq!(part2(&parse("()())")), Solution::Found(5));
    }
}
