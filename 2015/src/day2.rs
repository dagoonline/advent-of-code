use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(u32, u32, u32)> {
    let mut ans = vec![];
    for line in input.lines() {
        let parts: Vec<u32> = line.split("x").map(|x| x.parse().unwrap()).collect();
        ans.push((parts[0], parts[1], parts[2]));
    }

    ans
}

#[aoc(day2, part1)]
fn part1(input: &[(u32, u32, u32)]) -> u64 {
    let mut total: u64 = 0;
    for b in input {
        let side1 = b.0 * b.1;
        let side2 = b.0 * b.2;
        let side3 = b.1 * b.2;
        let smallest = side1.min(side2.min(side3));

        let partial = 2 * side1 + 2 * side2 + 2 * side3 + smallest;
        total += u64::from(partial);
        println!(
            "side1: {}, side2: {}, side3: {}, smallest: {}, partial: {}, total: {}",
            side1, side2, side3, smallest, partial, total
        )
    }

    total
}

#[aoc(day2, part2)]
fn part2(input: &[(u32, u32, u32)]) -> u64 {
    let mut total: u64 = 0;
    for b in input {
        let biggest = b.0.max(b.1.max(b.2));
        let length = 2 * (b.0 + b.1 + b.2) - 2 * biggest;

        let partial = length + b.0 * b.1 * b.2;
        total += u64::from(partial);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("2x3x4")), 58);
        assert_eq!(part1(&parse("1x1x10")), 43);
        assert_eq!(part1(&parse("3x11x24")), 771);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("2x3x4")), 34);
        assert_eq!(part2(&parse("1x1x10")), 14);
    }
}
