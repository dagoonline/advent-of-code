use itertools::Itertools;

#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {
    const SEARCH: &str = "XMAS";

    let mut input = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let rows = input.len();
    let cols = input[0].len();

    let mut transposed: Vec<Vec<_>> = (0..cols)
        .map(|col| (0..rows).map(|row| input[row][col]).collect())
        .collect();

    let mut search_space = vec![];
    input.iter_mut().for_each(|line| {
        search_space.push(line.clone());
        line.reverse();
        search_space.push(line.to_vec());
    });
    transposed.iter_mut().for_each(|line| {
        search_space.push(line.clone());
        line.reverse();
        search_space.push(line.to_vec());
    });

    diagonal::diagonal_pos_pos(&input)
        .into_iter()
        .map(|item| item.into_iter().cloned().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .iter_mut()
        .for_each(|line| {
            search_space.push(line.clone());
            line.reverse();
            search_space.push(line.to_vec());
        });
    diagonal::diagonal_pos_neg(&input)
        .into_iter()
        .map(|item| item.into_iter().cloned().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .iter_mut()
        .for_each(|line| {
            search_space.push(line.clone());
            line.reverse();
            search_space.push(line.to_vec());
        });

    search_space.iter().fold(0, |acc, item| {
        let s = item.iter().join("");
        acc + s.matches(SEARCH).count() as i32
    })
}
fn check(input: &[Vec<char>], x: usize, y: usize) -> bool {
    input[y - 1][x - 1] != input[y + 1][x + 1]
        && input[y - 1][x - 1] as u16
            + input[y + 1][x + 1] as u16
            + input[y - 1][x + 1] as u16
            + input[y + 1][x - 1] as u16
            == ('M' as u16) * 2 + ('S' as u16) * 2
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    let input = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let rows = input.len();
    let cols = input[0].len();

    input
        .iter()
        .enumerate()
        .fold(
            vec![],
            |mut acc: Vec<(usize, usize)>, (y, value): (usize, &Vec<char>)| {
                acc.append(
                    &mut value
                        .iter()
                        .enumerate()
                        .filter(|(x, &value)| {
                            value == 'A' && y > 0 && *x > 0 && *x < cols - 1 && y < rows - 1
                        })
                        .map(|(x, _)| (x, y))
                        .collect_vec(),
                );
                acc
            },
        ) // Vec with positions of 'A'
        .iter()
        .fold(0, |acc, (x, y)| acc + check(&input, *x, *y) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 9);
    }
}
