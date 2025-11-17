use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3, part1)]
fn parse(input: &str) -> HashSet<(i16, i16)> {
    let mut houses = HashSet::new();
    let mut position = (0, 0);
    houses.insert(position);
    for c in input.chars() {
        match c {
            '>' => {
                position.0 += 1;
            }
            '<' => {
                position.0 -= 1;
            }
            '^' => {
                position.1 -= 1;
            }
            _ => {
                position.1 += 1;
            }
        }

        houses.insert(position);
    }
    houses
}

type Paths = (HashSet<(i16, i16)>, HashSet<(i16, i16)>);

#[aoc_generator(day3, part2)]
fn parse2(input: &str) -> Paths {
    let mut santa = String::new();
    let mut rsanta = String::new();

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            santa.push(c)
        } else {
            rsanta.push(c)
        }
    }

    (parse(&santa), parse(&rsanta))
}

#[aoc(day3, part1)]
fn part1(path: &HashSet<(i16, i16)>) -> u32 {
    path.len() as u32
}

#[aoc(day3, part2)]
fn part2(paths: &Paths) -> u32 {
    let (santa, rsanta) = paths;
    santa.union(rsanta).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(">")), 2);
        assert_eq!(part1(&parse("^>v<")), 4);
        assert_eq!(part1(&parse("^v^v^v^v^v")), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2("^v")), 3);
        assert_eq!(part2(&parse2("^>v<")), 3);
        assert_eq!(part2(&parse2("^v^v^v^v^v")), 11);
    }
}
