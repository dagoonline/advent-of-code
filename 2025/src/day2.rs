use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    let mut ranges = vec![];
    let parts = input.split(',');

    for part in parts {
        let mut range = part.split('-');
        let start: u64 = range.next().unwrap().parse().unwrap();
        let end: u64 = range.next().unwrap().parse().unwrap();

        ranges.push((start, end));
    }

    ranges
}

fn is_valid_part1(n: u64) -> bool {
    let s = n.to_string();
    let (first, second) = s.split_at(s.len() / 2);

    first != second
}

fn is_valid_part2(n: u64) -> bool {
    let s = n.to_string();
    let mut chunk_size = 1;

    while chunk_size <= s.len() / 2 {
        let first_chunk = &s[0..chunk_size];
        let mut from = chunk_size;
        let mut is_valid = false;

        while from < s.len() {
            let to = (from + chunk_size).min(s.len());
            let chunk = &s[from..to];

            if chunk != first_chunk {
                is_valid = true;
                break;
            }

            from += chunk_size
        }
        if !is_valid {
            return false;
        }

        chunk_size += 1
    }

    true
}

fn solver(input: &[(u64, u64)], is_valid: &dyn Fn(u64) -> bool) -> u64 {
    input.iter().fold(0, |acc, (start, end)| {
        acc + (*start..*end + 1).filter(|v| !is_valid(*v)).sum::<u64>()
    })
}

#[aoc(day2, part1)]
fn part1(input: &[(u64, u64)]) -> u64 {
    solver(input, &is_valid_part1)
}

#[aoc(day2, part2)]
fn part2(input: &[(u64, u64)]) -> u64 {
    solver(input, &is_valid_part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            )),
            1227775554
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            )),
            4174379265
        );
    }
}
