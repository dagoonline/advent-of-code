use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<u8> {
    let mut containers = vec![];
    for line in input.lines() {
        containers.push(line.trim().parse().unwrap())
    }
    containers
}

fn fill(input: &[u8], amount: u8) -> u8 {
    if input.is_empty() {
        return 0;
    }

    let mut combinations = 0;
    let capacity = input[0];

    if capacity == amount {
        combinations += 1
    }

    combinations += fill(&input[1..], amount);
    if amount > capacity {
        combinations += fill(&input[1..], amount - capacity);
    }

    combinations
}

fn fill_count(input: &[u8], amount: u8) -> Vec<u8> {
    if input.is_empty() {
        return vec![];
    }

    let mut quantity = vec![];
    let capacity = input[0];

    if capacity == amount {
        quantity.push(1);
    }

    quantity.extend(fill_count(&input[1..], amount));

    if amount > capacity {
        quantity.extend(
            fill_count(&input[1..], amount - capacity)
                .into_iter()
                .map(|c| c + 1)
                .collect::<Vec<u8>>()
                .iter(),
        );
    }

    quantity
}

#[aoc(day17, part1)]
fn part1(input: &[u8]) -> u8 {
    fill(input, 150)
}

#[aoc(day17, part2)]
fn part2(input: &[u8]) -> u8 {
    let v = fill_count(input, 150);
    let min = v.iter().min().unwrap();
    v.iter().filter(|&v| v == min).count() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            fill(
                &parse(
                    r#" 20
                    15
                    10
                    5
                    5"#
                ),
                25
            ),
            4
        );
    }

    #[test]
    fn part2_example() {
        let v = fill_count(
            &parse(
                r#" 20
                    15
                    10
                    5
                    5"#,
            ),
            25,
        );
        let min = v.iter().min().unwrap();
        assert_eq!(v.iter().filter(|&v| v == min).count(), 3);
    }
}
