use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Col> {
    let mut lines = input.lines().rev();
    let mut cols = Col::from_operations(lines.next().unwrap());

    for line in lines {
        cols.iter_mut().fold(0, |acc, c| c.add_value(line, acc));
    }

    cols
}

#[derive(Debug)]
struct Col {
    values: Vec<String>,
    operation: char,
    width: usize,
}

impl Col {
    fn from_operations(ops: &str) -> Vec<Col> {
        let mut chars = ops.chars();
        let mut result = vec![];

        let mut col = Col {
            values: vec![],
            operation: chars.next().unwrap(),
            width: 1,
        };

        for c in chars {
            match c {
                ' ' => col.width += 1,
                v => {
                    result.push(col);
                    col = Col {
                        values: vec![],
                        operation: v,
                        width: 1,
                    }
                }
            }
        }
        col.width += 1;
        result.push(col);

        result
    }

    fn add_value(&mut self, value: &str, from: usize) -> usize {
        let to = from + self.width - 1;

        self.values.push(value[from..to].to_string());

        to + 1
    }

    fn compute(&self) -> u64 {
        if self.operation == '+' {
            self.values
                .iter()
                .map(|s| s.trim().parse::<u64>().unwrap())
                .sum()
        } else {
            self.values
                .iter()
                .fold(1, |acc, v| acc * v.trim().parse::<u64>().unwrap())
        }
    }

    fn transpose(&self) -> Self {
        let mut result = vec![];
        result.resize(self.width - 1, String::new());

        for (i, r) in result.iter_mut().enumerate() {
            for v in self.values.iter() {
                let c = v.chars().nth(i).unwrap();
                if c != ' ' {
                    r.push(c)
                }
            }
        }

        for r in result.iter_mut() {
            *r = r.chars().rev().collect()
        }

        Col {
            width: result.len(),
            values: result,
            ..*self
        }
    }
}

#[aoc(day6, part1)]
fn part1(input: &[Col]) -> u64 {
    input.iter().map(Col::compute).sum()
}

#[aoc(day6, part2)]
fn part2(input: &[Col]) -> u64 {
    input.iter().map(|c| c.transpose().compute()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 4277556);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 3263827);
    }
}
