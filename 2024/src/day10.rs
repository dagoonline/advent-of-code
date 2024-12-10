use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;

#[aoc_generator(day10)]
fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<(usize, usize)>) {
    let mut board = vec![];
    let mut starting_points = vec![];
    let border = std::iter::repeat(-1);
    board.push(Vec::new());

    for (y, line) in input.lines().enumerate() {
        let mut new_line = vec![];
        new_line.push(-1);
        for (x, digit) in line.chars().enumerate() {
            let digit = digit.to_digit(10).unwrap() as i32;
            new_line.push(digit);
            if digit == 0 {
                starting_points.push((x + 1, y + 1))
            }
        }
        new_line.push(-1);
        board.push(new_line);
    }
    board[0] = border.clone().take(board[1].len()).collect();
    board.push(border.take(board[1].len()).collect());

    (board, starting_points)
}

#[aoc(day10, part1)]
fn part1((board, starting_points): &(Vec<Vec<i32>>, Vec<(usize, usize)>)) -> usize {
    starting_points.iter().fold(0, |acc, &point| {
        let results = &mut BTreeSet::new();
        search_depth(board, point, results);
        acc + results.len()
    })
}

fn search_depth(
    board: &Vec<Vec<i32>>,
    (x, y): (usize, usize),
    results: &mut BTreeSet<(usize, usize)>,
) {
    if board[y][x] == 9 {
        results.insert((x, y));
    } else {
        for dy in [-1, 1] {
            let ny = (y as i32 + dy) as usize;
            if board[ny][x] == board[y][x] + 1 {
                search_depth(board, (x, ny), results)
            }
        }

        for dx in [-1, 1] {
            let nx = (x as i32 + dx) as usize;
            if board[y][nx] == board[y][x] + 1 {
                search_depth(board, (nx, y), results)
            }
        }
    }
}

// #[aoc(day10, part2)]
// fn part2(input: &str) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 36);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse(EXAMPLE)), 36);
    // }
}
