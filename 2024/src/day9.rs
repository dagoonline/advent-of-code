use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<i64> {
    let mut chars = input.chars();
    let mut blocks = vec![];
    let mut id = 0;
    while let Some(block_space) = chars.next() {
        for _ in 0..block_space.to_digit(10).unwrap() {
            blocks.push(id)
        }
        if let Some(free_space) = chars.next() {
            for _ in 0..free_space.to_digit(10).unwrap() {
                blocks.push(-1)
            }
        }
        id += 1;
    }

    blocks
}

#[aoc(day9, part1)]
fn part1(input: &Vec<i64>) -> i64 {
    let compacted = &mut input.clone();
    let mut free_space_pointer = 0;
    let mut block_space_pointer = compacted.len() - 1;

    while block_space_pointer > free_space_pointer {
        while block_space_pointer > 0 && compacted[block_space_pointer] < 0 {
            block_space_pointer -= 1;
        }
        while free_space_pointer < compacted.len() && compacted[free_space_pointer] >= 0 {
            free_space_pointer += 1;
        }

        compacted[free_space_pointer] = compacted[block_space_pointer];
        compacted[block_space_pointer] = -1;
    }
    checksum(&compacted[..free_space_pointer])
}

fn checksum(v: &[i64]) -> i64 {
    let mut checksum = 0;
    for i in 0..v.len() as i64 {
        if v[i as usize] >= 0 {
            checksum += i * v[i as usize];
        }
    }
    checksum
}

// #[aoc(day9, part2)]
fn part2(_input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1928);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
