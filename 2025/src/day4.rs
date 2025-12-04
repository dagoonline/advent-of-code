use aoc_runner_derive::aoc;

#[derive(Default, Debug, Clone)]
struct Grid {
    height: usize,
    width: usize,
    rolls: Vec<Vec<bool>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut g = Grid::default();

        for line in value.lines() {
            g.width = line.len();
            g.height += 1;
            g.rolls.push(line.chars().map(|c| c == '@').collect())
        }

        g
    }
}

impl Grid {
    fn step(&mut self) -> u16 {
        let snapshot = self.rolls.clone();

        let mut rolls = 0;
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

        for (y, row) in snapshot.iter().enumerate() {
            for (x, &current_on) in row.iter().enumerate() {
                let mut neighbors = 0;
                for (dx, dy) in around {
                    let (x, y) = (x as i16, y as i16);

                    if x + dx >= 0
                        && x + dx < self.width as i16
                        && y + dy >= 0
                        && y + dy < self.height as i16
                        && snapshot[(y + dy) as usize][(x + dx) as usize]
                    {
                        neighbors += 1
                    }
                }

                if current_on && neighbors < 4 {
                    rolls += 1;
                    self.rolls[y][x] = false
                }
            }
        }

        rolls
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u16 {
    let mut g: Grid = input.into();

    g.step()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u16 {
    let mut g: Grid = input.into();
    let mut acc = 0;

    while let next = g.step()
        && next != 0
    {
        acc += next
    }

    acc
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
