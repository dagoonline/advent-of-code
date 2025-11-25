use aoc_runner_derive::aoc;

fn counter(input: &str) -> String {
    let mut chars = input.chars();
    let mut result = String::new();

    let mut current = chars.next().unwrap();
    let mut counter = 1;

    for c in chars {
        if current == c {
            counter += 1;
        } else {
            result += &format!("{counter}{current}");
            current = c;
            counter = 1;
        }
    }

    result += &format!("{counter}{current}");
    result
}

#[aoc(day10, part1)]
fn part1(input: &str) -> u32 {
    let mut output = input.to_owned();
    for _ in 0..40 {
        output = counter(&output)
    }
    output.len() as u32
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u32 {
    let mut output = input.to_owned();
    for _ in 0..50 {
        output = counter(&output)
    }
    output.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        assert_eq!(counter("1"), "11");
        assert_eq!(counter("11"), "21");
        assert_eq!(counter("1211"), "111221");
        assert_eq!(counter("111221"), "312211");
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("1113122113")), "<RESULT>");
    // }
}
