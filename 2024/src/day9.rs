use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9, part1)]
fn parse_part1(input: &str) -> Vec<i64> {
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

    loop {
        while block_space_pointer > 0 && compacted[block_space_pointer] < 0 {
            block_space_pointer -= 1;
        }
        while free_space_pointer < compacted.len() && compacted[free_space_pointer] >= 0 {
            free_space_pointer += 1;
        }

        if block_space_pointer <= free_space_pointer {
            break;
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

#[aoc_generator(day9, part2)]
fn parse_part2(input: &str) -> (VecDeque<(u8, u8)>, VecDeque<(u8, u8)>) {
    let mut chars = input.chars();
    let mut file_blocks: VecDeque<(u8, u8)> = VecDeque::new();
    let mut free_blocks: VecDeque<(u8, u8)> = VecDeque::new();

    let mut id = 0;
    while let Some(file_block) = chars.next() {
        file_blocks.push_back((id, file_block.to_digit(10).unwrap() as u8));

        if let Some(free_space) = chars.next() {
            free_blocks.push_back((id, free_space.to_digit(10).unwrap() as u8));
        }

        id += 1;
    }

    (file_blocks, free_blocks)
}

#[aoc(day9, part2)]
fn part2((file_blocks, free_blocks): &(VecDeque<(u8, u8)>, VecDeque<(u8, u8)>)) -> i64 {
    let mut consolidated = vec![];
    let mut current_file_block = 

    for (id, free_space) in free_blocks {
        for _ in 0..*free_space {

        }
    }

    part1(&blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE)), 2858);
    }
}
