use aoc_runner_derive::aoc;
fn count_chars(line: &str) -> u32 {
    let line = &line[1..line.len() - 1];
    let mut counter = 0;
    let mut index = 0;
    while index < line.len() {
        let c = line.as_bytes()[index] as char;
        if c == '\\' && index < line.len() - 1 {
            let nc = line.as_bytes()[index + 1] as char;
            if nc == '"' || nc == '\\' {
                index += 1
            } else if nc == 'x' && index < line.len() - 3 {
                let first = line.as_bytes()[index + 2];
                let second = line.as_bytes()[index + 3];
                if first.is_ascii_hexdigit() && second.is_ascii_hexdigit() {
                    index += 3
                }
            }
        }
        counter += 1;
        index += 1;
    }

    counter as u32
}

#[aoc(day8, part1)]
fn part1(input: &str) -> u32 {
    let mut acc = 0;
    let mut total = 0;
    for line in input.lines() {
        total += line.len() as u32;
        acc += count_chars(line);
    }

    total - acc
}

fn expand_cound_chars(line: &str) -> u32 {
    let mut index = 0;
    let mut counter = 2;
    while index < line.len() {
        let c = line.as_bytes()[index] as char;
        if c == '\\' || c == '"' {
            counter += 1;
        };
        counter += 1;
        index += 1;
    }

    counter as u32
}

#[aoc(day8, part2)]
fn part2(input: &str) -> u32 {
    let mut acc = 0;
    let mut total = 0;
    for line in input.lines() {
        total += line.len() as u32;
        acc += expand_cound_chars(line);
    }

    acc - total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("\"nnpipxufvbfpoz\\\"jno\""), 1);
        assert_eq!(part1("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\""), 12);
        assert_eq!(part1("\"j\""), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("\"\""), 4);
        assert_eq!(part2("\"abc\""), 4);
        assert_eq!(part2("\"aaa\\\"aaa\""), 6);
        assert_eq!(part2("\"\\x27\""), 5);
    }
}
