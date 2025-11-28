use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Data = (HashMap<(String, String), i32>, HashSet<String>);

#[aoc_generator(day13)]
fn parse(input: &str) -> Data {
    let mut happiness = HashMap::new();
    let mut people = HashSet::new();
    let re = regex::Regex::new(r"(?P<from>\w+) would (?P<gainlose>gain|lose) (?P<points>\d+) happiness units by sitting next to (?P<to>\w+)").unwrap();

    for line in input.lines() {
        let data = re.captures(line).unwrap();
        let points = data["points"].parse::<i32>().unwrap();
        let value = if &data["gainlose"] == "gain" {
            points
        } else {
            -points
        };
        happiness.insert((data["from"].to_owned(), data["to"].to_owned()), value);
        people.insert(data["from"].to_owned());
    }

    (happiness, people)
}

fn maximize_happiness(
    input: &Data,
    visited: &mut Vec<String>,
    pending: &HashSet<String>,
    acc: i32,
) -> i32 {
    if pending.len() == 1 {
        let p = pending.iter().next().unwrap().to_owned();
        let first = visited.first().unwrap().to_owned();
        let last = visited.last().unwrap().to_owned();
        let value = acc
            + input.0[&(p.clone(), first.clone())]
            + input.0[&(first, p.clone())]
            + input.0[&(last.clone(), p.clone())]
            + input.0[&(p.clone(), last)];
        println!("Finish: {visited:?} + {p}: {value}");
        return value;
    }

    let from = visited.last().unwrap().to_owned();
    let mut max = i32::MIN;
    let mut process = pending.clone();

    for person in process.clone() {
        process.remove(&person);
        visited.push(person.to_owned());
        let value = maximize_happiness(
            input,
            visited,
            &process,
            acc + input.0[&(from.clone(), person.to_owned())]
                + input.0[&(person.to_owned(), from.clone())],
        );
        visited.pop();
        process.insert(person.to_owned());

        if value > max {
            max = value
        }
    }
    max
}

#[aoc(day13, part1)]
fn part1(input: &Data) -> i32 {
    let mut list: Vec<String> = input.1.clone().into_iter().collect();
    let first = list.pop().unwrap();
    let pending = HashSet::from_iter(list);
    let mut visited = vec![first];

    maximize_happiness(input, &mut visited, &pending, 0)
}

#[aoc(day13, part2)]
fn part2(input: &Data) -> i32 {
    let mut input = input.clone();
    for person in input.1.clone() {
        input.0.insert(("Me".to_string(), person.clone()), 0);
        input.0.insert((person, "Me".to_string()), 0);
    }
    input.1.insert("Me".to_string());
    part1(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r#"Alice would gain 54 happiness units by sitting next to Bob.
                Alice would lose 79 happiness units by sitting next to Carol.
                Alice would lose 2 happiness units by sitting next to David.
                Bob would gain 83 happiness units by sitting next to Alice.
                Bob would lose 7 happiness units by sitting next to Carol.
                Bob would lose 63 happiness units by sitting next to David.
                Carol would lose 62 happiness units by sitting next to Alice.
                Carol would gain 60 happiness units by sitting next to Bob.
                Carol would gain 55 happiness units by sitting next to David.
                David would gain 46 happiness units by sitting next to Alice.
                David would lose 7 happiness units by sitting next to Bob.
                David would gain 41 happiness units by sitting next to Carol."#
            )),
            330
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r#"Alice would gain 54 happiness units by sitting next to Bob.
                Alice would lose 79 happiness units by sitting next to Carol.
                Alice would lose 2 happiness units by sitting next to David.
                Bob would gain 83 happiness units by sitting next to Alice.
                Bob would lose 7 happiness units by sitting next to Carol.
                Bob would lose 63 happiness units by sitting next to David.
                Carol would lose 62 happiness units by sitting next to Alice.
                Carol would gain 60 happiness units by sitting next to Bob.
                Carol would gain 55 happiness units by sitting next to David.
                David would gain 46 happiness units by sitting next to Alice.
                David would lose 7 happiness units by sitting next to Bob.
                David would gain 41 happiness units by sitting next to Carol."#
            )),
            286
        );
    }
}
