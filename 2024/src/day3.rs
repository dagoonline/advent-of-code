use lazy_static::lazy_static;
use regex::Regex;

#[aoc(day03, part1)]
fn part1(input: &str) -> i32 {
    parse_mul(input)
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
}

fn parse_mul(s: &str) -> i32 {
    RE.captures_iter(s)
        .map(|c| c.extract())
        .map(|(_, [s1, s2])| s1.parse::<i32>().unwrap() * s2.parse::<i32>().unwrap())
        .fold(0, |acc, x| x + acc)
}

#[aoc(day03, part2)]
fn part2(input: &str) -> i32 {
    // No REGEX challenge
    const DO_STR: &str = "do()";
    const DONT_STR: &str = "don't()";

    let next_do = |s: &str| s.find(DO_STR).unwrap_or(s.len());
    let next_dont = |s: &str| s.find(DONT_STR).unwrap_or(s.len());
    let next_part =
        |s: &mut String, n: usize| s.drain(..usize::min(s.len(), n)).collect::<String>();

    let mut chunk = input.to_string();
    let mut count = 0;
    while !chunk.is_empty() {
        let valid = next_dont(&chunk) + DONT_STR.len();
        count += parse_mul(&next_part(&mut chunk, valid));

        let not_valid = next_do(&chunk) + DO_STR.len();
        _ = next_part(&mut chunk, not_valid);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::parse_mul;
    const PART1_TEST: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    #[test]
    fn part01() {
        assert_eq!(parse_mul(PART1_TEST), 161);
    }

    use super::part2;
    const PART2_TEST: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn part02_test_1() {
        assert_eq!(part2(PART2_TEST), 48);
    }
}
