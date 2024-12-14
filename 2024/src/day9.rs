use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9, part1)]
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
fn part1(input: &[i64]) -> i64 {
    let compacted = &mut input.to_owned();
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

    compacted
        .iter()
        .enumerate()
        .filter(|(_, &n)| n >= 0)
        .map(|(number, position)| position * number as i64)
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
    let mut chars = input.chars();
    let mut file_blocks = vec![];
    let mut free_blocks = vec![];
    let mut id: u64 = 0;
    let mut position: usize = 0;
    while let Some(block_space) = chars.next() {
        let file_space = block_space.to_digit(10).unwrap() as usize;
        file_blocks.push((id, position..position + file_space));
        position += file_space;
        id += 1;

        if let Some(free_space) = chars.next() {
            let free_space = free_space.to_digit(10).unwrap() as usize;
            free_blocks.push(position..position + free_space);
            position += free_space;
        }
    }

    for file in file_blocks.iter_mut().rev() {
        if let Some((pos, free_space)) =
            free_blocks.iter_mut().enumerate().find(|(_, free_space)| {
                free_space.end <= file.1.start && free_space.len() >= file.1.len()
            })
        {
            let size = file.1.len() as u64;
            file.1 = free_space.start..free_space.start + file.1.len();
            *free_space = free_space.start + size as usize..free_space.end;
            #[allow(unstable_name_collisions)]
            if free_space.is_empty() {
                free_blocks.remove(pos);
            }
        }
    }

    file_blocks
        .into_iter()
        .map(|(number, positions)| positions.clone().sum::<usize>() as u64 * number)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1928);
        assert_eq!(part1(&parse("999")), 117);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2858);
        assert_eq!(part2("99999"), 432);
    }
}
