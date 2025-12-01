use aoc_runner_derive::aoc;

#[derive(Default, Debug, Clone)]
struct Grid {
    height: usize,
    width: usize,
    fixed_lights: Vec<(usize, usize)>,
    lights: Vec<Vec<bool>>,
}

impl Grid {
    fn step(&mut self) {
        let snapshot = self.lights.clone();

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
                let mut neighbors_on = 0;
                for (dx, dy) in around {
                    let (x, y) = (x as i16, y as i16);

                    if x + dx >= 0
                        && x + dx < self.width as i16
                        && y + dy >= 0
                        && y + dy < self.height as i16
                        && snapshot[(y + dy) as usize][(x + dx) as usize]
                    {
                        neighbors_on += 1
                    }
                }

                if current_on {
                    if !(neighbors_on == 2 || neighbors_on == 3) {
                        self.lights[y][x] = false;
                    }
                } else if neighbors_on == 3 {
                    self.lights[y][x] = true;
                }
            }
        }

        for (fx, fy) in &self.fixed_lights {
            self.lights[*fy][*fx] = true
        }
    }

    fn always_on(&mut self, fixed: Vec<(usize, usize)>) {
        self.fixed_lights = fixed;
        for (fx, fy) in &self.fixed_lights {
            self.lights[*fy][*fx] = true
        }
    }

    fn count_on(&self) -> u16 {
        self.lights
            .iter()
            .map(|v| v.iter().filter(|&v| *v).count() as u16)
            .sum()
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut g = Grid::default();

        for line in value.lines() {
            g.width = line.len();
            g.height += 1;
            g.lights.push(line.chars().map(|c| c == '#').collect())
        }

        g
    }
}

#[aoc(day18, part1)]
fn part1(input: &str) -> u16 {
    let mut g: Grid = input.into();
    for _ in 0..100 {
        g.step()
    }
    g.count_on()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> u16 {
    let mut g: Grid = input.into();

    g.always_on(vec![
        (0, 0),
        (g.width - 1, 0),
        (g.width - 1, g.height - 1),
        (0, g.height - 1),
    ]);

    for _ in 0..100 {
        g.step()
    }
    g.count_on()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 7);
    }
}
