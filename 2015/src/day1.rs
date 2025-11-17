use std::fmt::{self, Display};

use aoc_runner_derive::aoc;
#[aoc(day1, part1)]
fn part1(input: &str) -> i16 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        _ => acc - 1,
    })
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
fn part2(input: &str) -> Solution {
    let mut floor = 0;

    for (index, movement) in input.chars().enumerate() {
        floor += match movement {
            '(' => 1,
            _ => -1,
        };
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
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(")"), Solution::Found(1));
        assert_eq!(part2("()())"), Solution::Found(5));
    }
}
