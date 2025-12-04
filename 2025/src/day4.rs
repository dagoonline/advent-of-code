use aoc_runner_derive::aoc;

fn parse(input: &str) -> Vec<Vec<bool>> {
    let mut grid = vec![];

    for line in input.lines() {
        grid.push(line.chars().map(|c| c == '@').collect())
    }

    grid
}

fn get_acceseable(rolls: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let height = rolls.len();
    let width = rolls[0].len();

    let mut acceseables = vec![];
    let around: [(i16, i16); 8] = [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    for (y, row) in rolls.iter().enumerate() {
        for (x, &is_roll) in row.iter().enumerate() {
            if !is_roll {
                continue;
            }

            let mut neighbors = 0;

            for (dx, dy) in around {
                let (x, y) = (x as i16, y as i16);

                if x + dx >= 0
                    && x + dx < width as i16
                    && y + dy >= 0
                    && y + dy < height as i16
                    && rolls[(y + dy) as usize][(x + dx) as usize]
                {
                    neighbors += 1
                }
            }
            if neighbors < 4 {
                acceseables.push((x, y));
            }
        }
    }

    acceseables
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u16 {
    get_acceseable(&parse(input)).len() as u16
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u16 {
    let mut grid = parse(input);
    let mut acc = 0;

    while let next = get_acceseable(&grid)
        && !next.is_empty()
    {
        acc += next.len();

        for (x, y) in next.into_iter() {
            grid[y][x] = false;
        }
    }

    acc as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 43);
    }
}
