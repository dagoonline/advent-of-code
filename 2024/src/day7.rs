use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type BaseType = i64;
type Line = (BaseType, Vec<BaseType>);
type Lines = Vec<Line>;

enum Operation {
    Add,
    Mul,
}

struct Node {
    value: BaseType,
    next: BaseType,
    operation: Operation,
    position: usize,
}

impl Node {
    fn new(value: BaseType, next: BaseType, position: usize, operation: Operation) -> Self {
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
    input.iter().fold(0, |acc, line| acc + iterative(line))
}

#[aoc(day7, part2)]
fn part2(input: &Lines) -> BaseType {
    input.iter().fold(0, |acc, line| acc + recursive(line))
}

fn concat(n1: BaseType, n2: BaseType) -> BaseType {
    let mut size = 0;
    let mut remaining = n2;
    while remaining > 0 {
        size += 1;
        remaining /= 10;
    }
    n1 * i64::pow(10, size) + n2
}

fn iterative(line: &Line) -> BaseType {
    let remaining = &line.1;

    let mut todo = vec![];
    let position = remaining.len() - 1;
    let last = remaining[position];
    todo.push(Node::new(line.0, last, position, Operation::Add));
    todo.push(Node::new(line.0, last, position, Operation::Mul));

    while let Some(n) = todo.pop() {
        let value: BaseType;
        match n.operation {
            Operation::Mul => {
                if n.value % n.next == 0 {
                    value = n.value / n.next;
                    if value == 0 {
                        return line.0;
                    }
                } else {
                    continue;
                }
            }
            Operation::Add => {
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
                Operation::Add,
            ));
            todo.push(Node::new(
                value,
                remaining[n.position - 1],
                n.position - 1,
                Operation::Mul,
            ));
        }
    }

    0
}

fn recursive(line: &Line) -> i64 {
    if recursive_search(line.0, &line.1, line.1[0], 1) {
        return line.0;
    }
    0
}

fn recursive_search(total: BaseType, numbers: &Vec<BaseType>, acc: BaseType, pos: usize) -> bool {
    if pos == numbers.len() {
        return total == acc;
    }

    let number = numbers[pos];
    recursive_search(total, numbers, acc + number, pos + 1)
        || recursive_search(total, numbers, acc * number, pos + 1)
        || recursive_search(total, numbers, concat(acc, number), pos + 1)
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
        assert_eq!(part2(&parse(EXAMPLE)), 11387);
    }
}
