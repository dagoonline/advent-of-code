use aoc_runner_derive::{aoc, aoc_generator};

struct Coords(u64, u64);

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Coords> {
    let mut coords = Vec::new();

    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
        coords.push(Coords(x, y))
    }

    coords
}

#[aoc(day9, part1)]
fn part1(input: &[Coords]) -> u64 {
    let mut max_area = 0;

    for x in 0..input.len() {
        for y in x + 1..input.len() {
            let area =
                (input[x].0.abs_diff(input[y].0) + 1) * (input[x].1.abs_diff(input[y].1) + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

// #[aoc(day9, part2)]
// fn part2(input: &str) -> i64 {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 50);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse(INPUT)), 50);
    // }
}
