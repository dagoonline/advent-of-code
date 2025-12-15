use aoc_runner_derive::{aoc, aoc_generator};

struct Col {
    values: Vec<String>,
    operation: char,
    width: usize,
}

impl Col {
    fn new(operation: char, width: usize) -> Col {
        Col {
            values: vec![],
            operation,
            width,
        }
    }

    fn add(&mut self, value: String) {
        self.values.push(value);
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
        result.resize(self.width, String::new());

        for (i, r) in result.iter_mut().enumerate() {
            for v in self.values.iter() {
                let c = v.chars().nth(i).unwrap();

                if c != ' ' {
                    r.push(c)
                }
            }
        }

        for r in result.iter_mut() {
            *r = r.chars().collect()
        }

        Col {
            width: result.len(),
            values: result,
            ..*self
        }
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Col> {
    let mut cols = vec![];
    let mut lines = input.lines().rev();

    let last_line = lines.next().unwrap();
    let mut op = last_line[0..1].chars().next().unwrap();
    let mut start = 0;

    for (end, current) in last_line.chars().enumerate().skip(1) {
        if current != ' ' {
            cols.push(Col::new(op, end - start - 1));
            op = current;
            start = end;
        }
    }
    cols.push(Col::new(op, last_line.len() - start));

    for line in lines.rev() {
        let mut start = 0;
        for col in cols.iter_mut() {
            col.add(line[start..start + col.width].to_string());
            start += col.width + 1;
        }
    }

    cols
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
