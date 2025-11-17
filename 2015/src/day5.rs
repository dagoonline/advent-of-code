use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day5, part1)]
fn part1(input: &str) -> u32 {
    let mut nice_lines = 0;
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);

    for line in input.lines() {
        let (mut twice, mut forbidden) = (false, false);
        let mut vowel_count = 0;

        let mut previous = '#';
        for c in line.chars() {
            if vowels.contains(&c) {
                vowel_count += 1;
            }
            if c == previous {
                twice = true
            } else {
                match (previous, c) {
                    ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => forbidden = true,
                    _ => {}
                }
            }

            previous = c
        }

        if vowel_count >= 3 && twice && !forbidden {
            nice_lines += 1
        }
    }

    nice_lines
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    let mut nice_lines = 0;
    for line in input.lines() {
        let mut triplet = false;
        let mut pairs = false;
        let mut pairs_seen = HashSet::new();

        let mut previous1 = '#';
        let mut previous2 = '#';
        let mut previous3 = '#';
        for c in line.chars() {
            if c == previous2 {
                triplet = true;
            }

            if pairs_seen.contains(&(previous1, c))
                && !(c == previous1 && c == previous2 && c != previous3)
            {
                pairs = true
            }

            if !(c == previous1 && c == previous2) {
                pairs_seen.insert((previous1, c));
            }

            previous3 = previous2;
            previous2 = previous1;
            previous1 = c;
        }

        if triplet && pairs {
            nice_lines += 1
        }
    }

    nice_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("ugknbfddgicrmopn"), 1);
        assert_eq!(part1("aaa"), 1);
        assert_eq!(part1("jchzalrnumimnmhp"), 0);
        assert_eq!(part1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("aaaa"), 1);
        assert_eq!(part2("xxyxx"), 1);
        assert_eq!(part2("aaa"), 0);
        assert_eq!(part2("uurcxstgmygtbstg"), 0);
        assert_eq!(part2("ieodomkazucvgmuy"), 0);
        assert_eq!(part2("galwwwgugetdohkg"), 0);
    }
}
