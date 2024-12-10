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

    checksum(compacted)
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

#[aoc(day9, part2)]
fn part2(input: &Vec<i64>) -> i64 {
    let compacted = &mut input.clone();
    let mut current_index = compacted.len() - 1;
    while current_index > 0 {
        while current_index > 0 && compacted[current_index] == -1 {
            current_index -= 1;
        }
        let next_number = compacted[current_index];

        let mut next_number_size = 0;
        while current_index > 0 && compacted[current_index] == next_number {
            next_number_size += 1;
            current_index -= 1
        }
        let mut start = 0;
        while start < current_index {
            while start < compacted.len() && compacted[start] != -1 {
                start += 1;
            }
            let mut next_free_space_size = 0;
            let mut next_free_space_index = start;
            while start < compacted.len() && compacted[start] == -1 {
                next_free_space_size += 1;
                start += 1;
            }
            if next_number_size <= next_free_space_size
                && start - next_free_space_size < current_index
            {
                let mut positions_to_remove = next_number_size;
                let mut index_to_remove = current_index + 1;
                while next_number_size > 0 {
                    compacted[next_free_space_index] = next_number;
                    next_number_size -= 1;
                    next_free_space_index += 1;
                }
                while positions_to_remove > 0 {
                    compacted[index_to_remove] = -1;
                    index_to_remove += 1;
                    positions_to_remove -= 1;
                }
                break;
            }
        }
    }

    checksum(compacted)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1928);
        assert_eq!(part2(&parse("999")), 117);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 2858);
        assert_eq!(part2(&parse("999")), 117);
    }
}
