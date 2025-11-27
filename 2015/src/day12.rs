use aoc_runner_derive::aoc;

#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    let mut current = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut numbers: Vec<i32> = vec![];
    while current < chars.len() {
        if chars[current].is_ascii_digit() || chars[current] == '-' {
            let mut number = current + 1;
            while number < chars.len() && chars[number].is_ascii_digit() {
                number += 1;
            }
            if let Ok(n) = chars[current..number]
                .iter()
                .collect::<String>()
                .parse::<i32>()
            {
                numbers.push(n)
            }

            current = number;
        } else {
            current += 1
        }
    }
    numbers.iter().sum()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    let mut current = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut numbers: Vec<i32> = vec![];
    while current < chars.len() {
        if chars[current] == '{' {
            let mut next = current + 1;
            let mut red_found = false;
            let mut nested = 1;
            while nested != 0 {
                match chars[next] {
                    '{' => nested += 1,
                    '}' => nested -= 1,
                    ':' => {
                        if nested == 1
                            && next < chars.len() - 5
                            && chars[next..next + 5].iter().collect::<String>() == ":\"red"
                        {
                            red_found = true;
                            next += 4
                        }
                    }
                    _ => {}
                };
                next += 1;
            }
            if red_found {
                current = next;
                continue;
            }
        };

        if chars[current].is_ascii_digit() || chars[current] == '-' {
            let mut number = current + 1;
            while number < chars.len() && chars[number].is_ascii_digit() {
                number += 1;
            }
            if let Ok(n) = chars[current..number]
                .iter()
                .collect::<String>()
                .parse::<i32>()
            {
                numbers.push(n)
            }

            current = number;
        } else {
            current += 1
        }
    }
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("[1,2,3]"), 6);
        assert_eq!(part1("{\"a\":2,\"b\":4"), 6);
        assert_eq!(part1("[[[3]]]"), 3);
        assert_eq!(part1("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(part1("{\"a\":[-1,1]}"), 0);
        assert_eq!(part1("[]"), 0);
        assert_eq!(part1("{}"), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
        assert_eq!(part2("[1,2,3]"), 6);
        assert_eq!(part2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
        assert_eq!(part2("[1,\"red\",5]"), 6);
    }
}
