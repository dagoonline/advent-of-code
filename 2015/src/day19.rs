use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Data = (
    HashMap<String, Vec<String>>,
    String,
    HashMap<String, String>,
);

#[aoc_generator(day19)]
fn parse(input: &str) -> Data {
    let mut replacements = HashMap::new();
    let mut inverse_replacements = HashMap::new();
    let re = regex::Regex::new(r"(?P<key>\w+) => (?P<value>\w+)").unwrap();

    let mut last_line = false;
    let mut molecule = String::new();
    for line in input.lines() {
        if line.is_empty() {
            last_line = true;
            continue;
        }
        if last_line {
            molecule += line;
            continue;
        }
        let data = re.captures(line).unwrap();

        replacements
            .entry(data["key"].to_string())
            .and_modify(|v: &mut Vec<String>| v.push(data["value"].to_string()))
            .or_insert(vec![data["value"].to_string()]);
        inverse_replacements.insert(data["value"].to_string(), data["key"].to_string());
    }

    (replacements, molecule, inverse_replacements)
}

#[aoc(day19, part1)]
fn part1(input: &Data) -> usize {
    let replacements = input.0.clone();
    let molecule = input.1.clone();

    let mut new_molecules: HashSet<String> = HashSet::new();
    for key in replacements.keys().collect::<Vec<&String>>() {
        for replacement in input.0[key].clone() {
            let mut remaining = molecule.clone();
            let mut previous = String::new();
            while let Some(next) = remaining.find(key) {
                let (first, last) = remaining.split_at(next);
                previous += first;
                let new_molecule = previous.to_string() + &replacement + &last[key.len()..];
                new_molecules.insert(new_molecule);
                previous += key;
                remaining = last[key.len()..].to_string();
            }
        }
    }
    new_molecules.len()
}

#[aoc(day19, part2)]
fn part2(input: &Data) -> usize {
    let molecule = input.1.clone();
    let ireplacements = input.2.clone();

    let mut keys: Vec<String> = ireplacements.keys().cloned().collect();
    let mut rng = rand::rng();

    loop {
        // Get a random sequence of replacements and try to reduce the current molecule down to 'e'
        // Non deterministic for generic case, but works and very fast for the order of the problem input
        rand::seq::SliceRandom::shuffle(keys.as_mut_slice(), &mut rng);

        let mut temp = molecule.clone();
        let mut steps = 0;
        let mut stuck = false;

        while temp != "e" {
            let mut found = false;
            for key in &keys {
                if temp.contains(key.as_str()) {
                    temp = temp.replacen(key.as_str(), &ireplacements[key], 1);
                    steps += 1;
                    found = true;
                    break;
                }
            }
            if !found {
                stuck = true;
                break;
            }
        }

        if !stuck {
            return steps;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r"H => HO
H => OH
O => HH

HOH"
            )),
            4
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r"e => H
e => O
H => HO
H => OH
O => HH

HOH"
            )),
            3
        );
        assert_eq!(
            part2(&parse(
                r"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO"
            )),
            6
        );
    }
}
