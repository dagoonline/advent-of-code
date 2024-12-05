use std::collections::BTreeMap;

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut v1 = vec![];
    let mut v2 = vec![];

    input
        .lines()
        .map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()))
        .for_each(|mut item| {
            v1.push(item.next().unwrap());
            v2.push(item.next().unwrap())
        });

    v1.sort();
    v2.sort();
    (v1, v2)
}

#[aoc(day01, part1)]
fn part1((v1, v2): &(Vec<i32>, Vec<i32>)) -> i32 {
    distance(v1, v2)
}

#[aoc(day01, part2)]
fn part2((v1, v2): &(Vec<i32>, Vec<i32>)) -> i32 {
    similarity(v1, v2)
}

fn distance(v1: &[i32], v2: &[i32]) -> i32 {
    v1.iter()
        .zip(v2)
        .fold(0, |acc, (&x, y)| acc + i32::abs(x - y))
}

fn similarity(v1: &[i32], v2: &[i32]) -> i32 {
    let mut hm = BTreeMap::new();
    v2.iter().for_each(|item| match hm.get(item) {
        Some(count) => {
            hm.insert(item, count + 1);
        }
        None => {
            hm.insert(item, 1);
        }
    });

    let mut total = 0;
    v1.iter().for_each(|item| {
        if let Some(value) = hm.get(item) {
            total += item * value
        }
    });
    total
}

#[cfg(test)]
mod tests {

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    use super::*;
    #[test]
    fn test_base() {
        assert_eq!(part1(&parse(EXAMPLE)), 11);
    }

    use super::part2;
    #[test]
    fn test_base_similarity() {
        assert_eq!(part2(&parse(EXAMPLE)), 31);
    }
}
