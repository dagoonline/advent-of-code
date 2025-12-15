use aoc_runner_derive::{aoc, aoc_generator};
use foldhash::{HashMap, HashMapExt};

#[aoc_generator(day7)]
fn parse(input: &str) -> (usize, Vec<Vec<usize>>) {
    let mut lines = input.lines();

    let start = lines.next().unwrap().find('S').unwrap();
    let splitters: Vec<Vec<usize>> = lines
        .skip(1)
        .step_by(2)
        .map(|l| l.match_indices('^').map(|(i, _)| i).collect())
        .collect();

    (start, splitters)
}

fn merge(beams: &mut HashMap<usize, u64>, splitters: &Vec<usize>) -> u64 {
    let mut count = 0;

    for splitter in splitters {
        if beams.contains_key(splitter) {
            let value = beams.get(splitter).unwrap().to_owned();

            beams
                .entry(splitter - 1)
                .and_modify(|v| *v += value)
                .or_insert(value);
            beams
                .entry(splitter + 1)
                .and_modify(|v| *v += value)
                .or_insert(value);

            beams.remove(splitter);

            count += 1;
        }
    }

    count
}

#[aoc(day7, part1)]
fn part1(data: &(usize, Vec<Vec<usize>>)) -> u64 {
    let mut beams = HashMap::new();
    let mut splits = 0;

    beams.insert(data.0, 1);
    for splitters in data.1.iter() {
        splits += merge(&mut beams, splitters);
    }

    splits
}

#[aoc(day7, part2)]
fn part2(data: &(usize, Vec<Vec<usize>>)) -> u64 {
    let mut beams = HashMap::new();

    beams.insert(data.0, 1);
    for splitters in data.1.iter() {
        merge(&mut beams, splitters);
    }

    beams.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 40);
    }
}
