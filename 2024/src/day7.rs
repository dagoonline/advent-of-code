use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type BaseType = i64;
type Line = (BaseType, Vec<BaseType>);
type Lines = Vec<Line>;

enum Operations {
    Add,
    Mul,
}

struct Node {
    value: BaseType,
    next: BaseType,
    operation: Operations,
    position: usize,
}

impl Node {
    fn new(value: BaseType, next: BaseType, position: usize, operation: Operations) -> Self {
        Self {
            value,
            next,
            operation,
            position,
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Lines {
    input
        .lines()
        .map(|line| line.split(':').collect_vec())
        .map(|parts| {
            (
                parts[0].parse().unwrap(),
                parts[1]
                    .split_whitespace()
                    .map(|element| element.parse().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
}

#[aoc(day7, part1)]
fn part1(input: &Lines) -> BaseType {
    input.iter().fold(0, |acc, line| acc + add_combinable(line))
}

fn add_combinable(line: &Line) -> BaseType {
    let remaining = &line.1;

    let mut todo = vec![];
    let position = remaining.len() - 1;
    let last = remaining[position];
    todo.push(Node::new(line.0, last, position, Operations::Add));
    todo.push(Node::new(line.0, last, position, Operations::Mul));

    while let Some(n) = todo.pop() {
        let value: BaseType;
        match n.operation {
            Operations::Mul => {
                if n.value % n.next == 0 {
                    value = n.value / n.next;
                    if value == 0 {
                        return line.0;
                    }
                } else {
                    continue;
                }
            }
            Operations::Add => {
                if n.value - n.next >= 0 {
                    value = n.value - n.next;
                    if value == 0 {
                        return line.0;
                    }
                } else {
                    continue;
                }
            }
        }
        if n.position != 0 {
            todo.push(Node::new(
                value,
                remaining[n.position - 1],
                n.position - 1,
                Operations::Add,
            ));
            todo.push(Node::new(
                value,
                remaining[n.position - 1],
                n.position - 1,
                Operations::Mul,
            ));
        }
    }

    0
}

#[aoc(day7, part2)]
fn part2(input: &Lines) -> BaseType {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
