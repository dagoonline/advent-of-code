use aoc_runner_derive::aoc;
use itertools::Itertools;

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = vec![];
    let mut lines = input.lines();

    while let line = lines.next().unwrap()
        && !line.is_empty()
    {
        let (s, e) = line
            .split('-')
            .map(|s| s.parse().unwrap())
            .next_tuple()
            .unwrap();

        ranges.push((s, e))
    }

    ranges.sort();

    let mut ids = vec![];
    for line in lines {
        ids.push(line.parse().unwrap())
    }

    (ranges, ids)
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u64 {
    let (ranges, ids) = parse(input);
    let mut count = 0;

    for i in ids {
        for (s, e) in ranges.iter() {
            // range is sorted
            if *s > i {
                break;
            }

            if *s <= i && i <= *e {
                count += 1;
                break;
            }
        }
    }

    count
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u64 {
    let (ranges, _) = parse(input);
    let ranges = ranges.into_iter();
    let (mut count, mut last) = (0, 0);

    for (s, e) in ranges {
        if s > last && e > last {
            count += e - s + 1;
            last = e;
        } else if e > last {
            count += e - last;
            last = e;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 14);
    }
}
