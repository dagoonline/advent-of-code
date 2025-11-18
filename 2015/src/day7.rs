use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Op {
    RShift(String, u16),
    LShift(String, u16),
    Not(String),
    And(String, String),
    Or(String, String),
    Signal(String),
}

#[aoc_generator(day7)]
fn parse(input: &str) -> HashMap<String, Op> {
    let mut circuit = HashMap::new();
    let reline = Regex::new(r"^(?P<operand>.+) -> (?P<wire>[\w]+)$").unwrap();

    for line in input.lines() {
        let capture = reline.captures(line).unwrap();
        let tag = capture["wire"].to_owned();
        let operand = capture["operand"].to_owned();

        if operand.contains("NOT") {
            circuit.insert(tag, Op::Not(operand[4..].to_owned()));
        } else if operand.contains("AND") {
            let values: Vec<&str> = operand.split(" AND ").collect();
            circuit.insert(tag, Op::And(values[0].to_owned(), values[1].to_owned()));
        } else if operand.contains("OR") {
            let values: Vec<&str> = operand.split(" OR ").collect();
            circuit.insert(tag, Op::Or(values[0].to_owned(), values[1].to_owned()));
        } else if operand.contains("LSHIFT") {
            let values: Vec<&str> = operand.split(" LSHIFT ").collect();
            circuit.insert(
                tag,
                Op::LShift(values[0].to_owned(), values[1].parse().unwrap()),
            );
        } else if operand.contains("RSHIFT") {
            let values: Vec<&str> = operand.split(" RSHIFT ").collect();
            circuit.insert(
                tag,
                Op::RShift(values[0].to_owned(), values[1].parse().unwrap()),
            );
        } else {
            circuit.insert(tag, Op::Signal(operand));
        }
    }
    circuit
}

fn resolver(input: &mut HashMap<String, Op>, tag: &str) -> u16 {
    let operation = &input.get_mut(tag).cloned();
    let result = if let Some(operation) = operation {
        match operation {
            Op::And(l, r) => resolver(input, l) & resolver(input, r),
            Op::Or(l, r) => resolver(input, l) | resolver(input, r),
            Op::Not(v) => !resolver(input, v),
            Op::RShift(v, shift) => resolver(input, v) >> *shift,
            Op::LShift(v, shift) => resolver(input, v) << *shift,
            Op::Signal(v) => resolver(input, v),
        }
    } else {
        tag.parse().unwrap()
    };
    input
        .entry(tag.to_string())
        .and_modify(|v| *v = Op::Signal(result.to_string()));
    result
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<String, Op>) -> u16 {
    resolver(&mut input.clone(), "a")
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<String, Op>) -> u16 {
    let mut input = input.clone();
    input
        .entry("b".to_owned())
        .and_modify(|v| *v = Op::Signal("46065".to_owned()));
    resolver(&mut input, "a")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(resolver(&mut parse("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), "d"), 72);
        assert_eq!(resolver(&mut parse("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), "e"), 507);
        assert_eq!(resolver(&mut parse("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), "f"), 492);
        assert_eq!(resolver(&mut parse("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), "g"), 114);
        assert_eq!(resolver(&mut parse("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), "h"), 65412);
        assert_eq!(resolver(&mut parse("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), "i"), 65079);
        assert_eq!(resolver(&mut parse("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), "x"), 123);
        assert_eq!(resolver(&mut parse("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), "y"), 456);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
