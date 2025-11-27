use aoc_runner_derive::aoc;
use itertools::Itertools;

fn has_increasing(input: &str) -> bool {
    for (a, b, c) in input.chars().tuple_windows::<(_, _, _)>() {
        let na = std::char::from_u32(a as u32 + 1).unwrap();
        let nb = std::char::from_u32(b as u32 + 1).unwrap();
        if c == nb && b == na {
            return true;
        }
    }

    false
}

fn has_forbidden_chars(input: &str) -> bool {
    input.contains('i') || input.contains('o') || input.contains('l')
}

fn different_pairs(input: &str) -> bool {
    let mut count = 0;
    let mut current = 1;
    let chars = input.chars().collect::<Vec<char>>();
    while current < chars.len() {
        if chars[current] == chars[current - 1] {
            count += 1;
            current += 1
        }
        if count == 2 {
            return true;
        }

        current += 1;
    }

    false
}

fn is_valid(input: &str) -> bool {
    has_increasing(input) && !has_forbidden_chars(input) && different_pairs(input)
}

fn next(input: &str) -> Option<String> {
    if input.is_empty() {
        return None;
    }

    let first = input.chars().next().unwrap();

    if has_forbidden_chars(&first.to_string()) {
        return Some(
            String::from(std::char::from_u32(first as u32 + 1).unwrap())
                + &input[1..].chars().map(|_| 'a').collect::<String>(),
        );
    }

    let result = if let Some(s) = next(&input[1..]) {
        String::from(first) + &s
    } else if first == 'z' {
        return None;
    } else {
        String::from(std::char::from_u32(first as u32 + 1).unwrap())
            + &input[1..].chars().map(|_| 'a').collect::<String>()
    };

    Some(result)
}

#[aoc(day11, part1)]
fn part1(input: &str) -> String {
    let mut result = input.to_string();

    while !is_valid(&result) {
        result = next(&result).unwrap();
    }

    result.to_owned()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> String {
    part1(&next(&part1(input)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_increasing() {
        assert!(has_increasing("abc"));
        assert!(has_increasing("cde"));
        assert!(has_increasing("xyz"));
        assert!(!has_increasing("abd"));
        assert!(!has_increasing("cba"));
    }

    #[test]
    fn test_no_forbidden_chars() {
        assert!(has_forbidden_chars("abc"));
        assert!(has_forbidden_chars("abcdefghj"));
        assert!(!has_forbidden_chars("aei"));
        assert!(!has_forbidden_chars("aeo"));
        assert!(!has_forbidden_chars("l"));
    }

    #[test]
    fn test_different_pairs() {
        assert!(different_pairs("aabb"));
        assert!(!different_pairs("aaabc"));
        assert!(different_pairs("abccdefgg"));
        assert!(!different_pairs("abacadafgg"));
        assert!(different_pairs("bacadafftinioo"));
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("abcdefgh"), "abcdffaa");
        assert_eq!(part1("ghijklmn"), "ghjaabcc");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("<EXAMPLE>"), "<RESULT>");
    }
}
