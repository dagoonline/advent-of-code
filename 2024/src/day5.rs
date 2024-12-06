use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Dependencies = HashMap<u32, HashSet<u32>>;
type Lines = Vec<Vec<u32>>;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Dependencies, Lines) {
    let mut dependencies = Dependencies::new();

    input
        .lines()
        .map(|line| line.split('|').collect::<Vec<&str>>())
        .filter(|v| v.len() == 2)
        .map(|parts| {
            [
                parts[0].parse::<u32>().unwrap(),
                parts[1].parse::<u32>().unwrap(),
            ]
        })
        .for_each(|[page, dependency]| {
            dependencies.entry(page).or_default().insert(dependency);
        });

    let inputs = input
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .filter(|v| v.len() > 1)
        .map(|line| {
            line.iter()
                .map(|&page| page.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (dependencies, inputs)
}

#[aoc(day5, part1)]
fn part1((deps, inputs): &(Dependencies, Lines)) -> u32 {
    inputs
        .iter()
        .filter(|&input| is_valid(deps, input))
        .fold(0, |acc, input| acc + input[input.len() / 2])
}

#[aoc(day5, part2)]
fn part2((deps, inputs): &(Dependencies, Lines)) -> u32 {
    inputs
        .iter()
        .filter(|input| !is_valid(deps, input))
        .map(|input| fix_it(deps, input))
        .fold(0, |acc, line| acc + line[line.len() / 2])
}

fn is_valid(deps: &Dependencies, line: &[u32]) -> bool {
    let mut processed = vec![];

    for page in line {
        if let Some(pages_before) = deps.get(page) {
            if pages_before.iter().any(|page| processed.contains(page)) {
                return false;
            }
        }
        processed.push(*page);
    }

    true
}

fn fix_it(deps: &Dependencies, pages: &[u32]) -> Vec<u32> {
    let mut sorted_pages = pages.to_owned();
    let mut processed_items = vec![];
    let mut fix_converged = false;

    while !fix_converged {
        fix_converged = true;
        for page in sorted_pages {
            let mut insert_position = processed_items.len();
            if let Some(dependant_pages) = deps.get(&page) {
                for dependant_page in dependant_pages {
                    if let Some(position) = processed_items.iter().position(|x| x == dependant_page)
                    {
                        insert_position = position;
                        fix_converged = false;
                        break;
                    }
                }
            }
            processed_items.insert(insert_position, page);
        }
        sorted_pages = processed_items;
        processed_items = vec![];
    }

    sorted_pages
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 123);
    }
}
