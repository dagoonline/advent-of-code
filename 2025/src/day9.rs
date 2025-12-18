use aoc_runner_derive::{aoc, aoc_generator};
use foldhash::{HashMap, HashMapExt, HashSet, HashSetExt};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    pub x: u64,
    pub y: u64,
}

impl Tile {
    pub fn new(x: u64, y: u64) -> Self {
        Tile { x, y }
    }
}

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Vec<Tile> {
    let mut coords = Vec::new();

    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

        coords.push(Tile::new(x, y));
    }

    coords
}

#[aoc(day9, part1)]
fn part1(input: &[Tile]) -> u64 {
    let mut max_area = 0;

    for s in 0..input.len() {
        for e in s + 1..input.len() {
            let area =
                (input[s].x.abs_diff(input[e].x) + 1) * (input[s].y.abs_diff(input[e].y) + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

pub fn perimeter(input: &[Tile]) -> HashSet<Tile> {
    let mut perimeter = HashSet::new();

    for pair in input.windows(2) {
        (pair[0].x..pair[1].x).for_each(|x| {
            perimeter.insert(Tile::new(x, pair[0].y));
        });
        (pair[1].x..pair[0].x).for_each(|x| {
            perimeter.insert(Tile::new(x, pair[1].y));
        });
        (pair[0].y..pair[1].y).for_each(|y| {
            perimeter.insert(Tile::new(pair[0].x, y));
        });
        (pair[1].y..pair[0].y).for_each(|y| {
            perimeter.insert(Tile::new(pair[1].x, y));
        });
    }
    let last = input.len() - 1;
    (input[0].x..input[last].x).for_each(|x| {
        perimeter.insert(Tile::new(x, input[0].y));
    });
    (input[last].x..input[0].x).for_each(|x| {
        perimeter.insert(Tile::new(x, input[last].y));
    });
    (input[0].y..input[last].y).for_each(|y| {
        perimeter.insert(Tile::new(input[0].x, y));
    });
    (input[last].y..input[0].y).for_each(|y| {
        perimeter.insert(Tile::new(input[last].x, y));
    });
    perimeter.insert(Tile::new(input[last].x, input[last].y));

    perimeter
}

pub fn is_inside(cache: &mut HashMap<Tile, bool>, tile: &Tile, perimeter: &HashSet<Tile>) -> bool {
    if perimeter.contains(tile) {
        return true;
    }
    if let Some(inside) = cache.get(tile) {
        return *inside;
    }

    let mut count = 0;
    let mut current = tile.clone();

    while current.x > 0 {
        if perimeter.contains(&current) {
            count += 1;
            while perimeter.contains(&current) {
                current.x -= 1
            }
        }

        current.x -= 1
    }

    cache.insert(tile.clone(), count % 2 == 1);

    *cache.get(tile).unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &[Tile]) -> u64 {
    let mut max = 0;
    let p = perimeter(input);
    let mut cache: HashMap<Tile, bool> = HashMap::new();
    for i in 0..input.len() {
        let tile1 = &input[i];
        for tile2 in input.iter().skip(i + 1) {
            let size = (tile1.x.abs_diff(tile2.x) + 1) * (tile1.y.abs_diff(tile2.y) + 1);

            if tile1.x != tile2.x
                && tile1.y != tile2.y
                && size > max
                && is_inside(&mut cache, &Tile::new(tile1.x, tile2.y), &p)
                && is_inside(&mut cache, &Tile::new(tile2.x, tile1.y), &p)
            {
                let new_perimeter = perimeter(&[
                    tile1.clone(),
                    Tile::new(tile1.x, tile2.y),
                    tile2.clone(),
                    Tile::new(tile2.x, tile1.y),
                ]);

                let mut valid = true;
                for tile in new_perimeter {
                    if !is_inside(&mut cache, &tile, &p) {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    max = size
                }
            }
        }
    }

    max
}

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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 24);
    }
}
