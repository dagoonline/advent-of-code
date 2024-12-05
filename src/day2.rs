use itertools::Itertools;

#[derive(PartialEq)]
enum Trend {
    Upwards,
    Downwards,
}

impl Trend {
    pub fn new(upward: bool) -> Trend {
        if upward {
            return Trend::Upwards;
        }
        Trend::Downwards
    }
    pub fn invalid(&self, (v1, v2): (&i32, &i32)) -> bool {
        self == &Self::Upwards && v1 >= v2
            || self == &Self::Downwards && v1 <= v2
            || i32::abs_diff(*v1, *v2) > 3
    }
}

#[aoc(day02, part1)]
fn part1(input: &str) -> usize {
    parse_apply(input, &validate)
}

#[aoc(day02, part2)]
fn part2(input: &str) -> usize {
    parse_apply(input, &validate_with_skip)
}

fn parse_apply(input: &str, f: &dyn Fn(&[i32]) -> bool) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .filter(|line| f(line))
        .count()
}

fn validate(v: &[i32]) -> bool {
    let trend = Trend::new(v[0] < v[1]);
    v.iter().tuple_windows().all(|tuple| !trend.invalid(tuple))
}

fn validate_with_skip(v: &[i32]) -> bool {
    if validate(v) {
        return true;
    }

    (0..v.len()).any(|skip| check_skip_col(v, skip))
}

fn check_skip_col(v: &[i32], n: usize) -> bool {
    validate(
        v.iter()
            .enumerate()
            .filter_map(|(i, &value)| if i == n { None } else { Some(value) })
            .collect_vec()
            .as_slice(),
    )
}

#[cfg(test)]
mod test {

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    use super::part1;
    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    use super::part2;
    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 4);
    }
}
