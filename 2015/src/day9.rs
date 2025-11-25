use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[aoc_generator(day9)]
fn parse(input: &str) -> HashMap<String, HashMap<String, u32>> {
    let re = Regex::new(r"(?P<from>[\w]+) to (?P<to>[\w]+) = (?P<distance>[\d]+)").unwrap();
    let mut matrix = HashMap::new();

    for line in input.lines() {
        let info = re.captures(line).unwrap();
        matrix
            .entry(info["from"].to_string())
            .or_insert(HashMap::new())
            .insert(
                info["to"].to_string(),
                info["distance"].parse::<u32>().unwrap(),
            );
        matrix
            .entry(info["to"].to_string())
            .or_insert(HashMap::new())
            .insert(
                info["from"].to_string(),
                info["distance"].parse::<u32>().unwrap(),
            );
    }

    matrix
}

fn min_path(
    input: &HashMap<String, HashMap<String, u32>>,
    from: &String,
    cities: &Vec<String>,
    visited: &mut HashSet<String>,
) -> u32 {
    if visited.len() == cities.len() {
        return 0;
    }

    visited.insert(from.to_owned());

    let mut min = u32::MAX;
    for city in cities {
        if !visited.contains(city) {
            let distance = input[from][city] + min_path(input, city, cities, visited);
            if distance < min {
                min = distance;
            }
        }
    }
    visited.remove(from);

    if min < u32::MAX {
        min
    } else {
        0
    }
}

fn max_path(
    input: &HashMap<String, HashMap<String, u32>>,
    from: &String,
    cities: &Vec<String>,
    visited: &mut HashSet<String>,
) -> u32 {
    if visited.len() == cities.len() {
        return 0;
    }

    visited.insert(from.to_owned());

    let mut max = 0;
    for city in cities {
        if !visited.contains(city) {
            let distance = input[from][city] + max_path(input, city, cities, visited);
            if distance > max {
                max = distance;
            }
        }
    }
    visited.remove(from);

    max
}

#[aoc(day9, part1)]
fn part1(input: &HashMap<String, HashMap<String, u32>>) -> u32 {
    let cities: Vec<String> = input.keys().cloned().collect();
    let mut visited = HashSet::new();
    cities
        .iter()
        .map(|city| min_path(input, city, &cities, &mut visited))
        .collect::<Vec<u32>>()
        .iter()
        .min()
        .unwrap()
        .to_owned()
}

#[aoc(day9, part2)]
fn part2(input: &HashMap<String, HashMap<String, u32>>) -> u32 {
    let cities: Vec<String> = input.keys().cloned().collect();
    let mut visited = HashSet::new();
    cities
        .iter()
        .map(|city| max_path(input, city, &cities, &mut visited))
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141"
            )),
            605
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141"
            )),
            982
        );
    }
}

// AC, Tris, Snowdin, Faerun, Tambi, Straylight, Norrath, Straylight, Abre
// 3 + 12 + 35 + 21 + 9 + 27 + 40
/*
AlphaCentauri to Tristram = 3
Norrath to Straylight = 9
Snowdin to Faerun = 12
Faerun to Straylight = 21
Tambi to Tristram = 35
Tambi to Norrath = 113

Norrath -> Straylight -> Faerun -> Tambi -> Tristram -> AlphaCentauri

Straylight to Tristram = 27
AlphaCentauri to Snowdin = 66
AlphaCentauri to Tambi = 28
AlphaCentauri to Faerun = 60
AlphaCentauri to Norrath = 34
AlphaCentauri to Straylight = 34
AlphaCentauri to Arbre = 108
Snowdin to Tambi = 22
Snowdin to Norrath = 91
Snowdin to Straylight = 121
Snowdin to Tristram = 111
Snowdin to Arbre = 71
Tambi to Straylight = 130
Tambi to Arbre = 40
Tambi to Faerun = 39
Faerun to Norrath = 63
Faerun to Tristram = 57
Faerun to Arbre = 83
Norrath to Tristram = 50
Norrath to Arbre = 60
Straylight to Arbre = 81
Tristram to Arbre = 90
 */
