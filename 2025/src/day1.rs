use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i16> {
    let mut rotations = vec![];

    for line in input.lines() {
        let direction = &line[0..1];
        let &value = &line[1..].parse::<i16>().unwrap();

        if direction == "L" {
            rotations.push(-value)
        } else {
            rotations.push(value)
        }
    }

    rotations
}

#[aoc(day1, part1)]
fn part1(input: &Vec<i16>) -> i16 {
    let mut count = 0;
    let mut dial: i16 = 50;

    for rotation in input {
        dial += rotation % 100;
        dial = dial.rem_euclid(100);
        if dial == 0 {
            count += 1
        }
    }

    count
}

#[aoc(day1, part2)]
fn part2(input: &Vec<i16>) -> i16 {
    let mut count = 0;
    let mut dial: i16 = 50;

    for rotation in input {
        let initial = dial;
        count += rotation.abs() / 100;
        dial += rotation % 100;
        if (initial != 0 && dial <= 0) || dial >= 100 {
            count += 1
        }
        dial = dial.rem_euclid(100);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 6);
    }
}
