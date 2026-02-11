use aoc_runner_derive::{aoc, aoc_generator};
use foldhash::{HashMap, HashMapExt};

#[aoc_generator(day11)]
fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (key, rest) = line.split_once(':').unwrap();
        for target in rest.split_whitespace() {
            graph
                .entry(key.to_string())
                .or_default()
                .push(target.into());
        }
    }

    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    checkpoints: &[&str],
    mut visited: u32,
    cache: &mut HashMap<(String, u32), u64>,
) -> u64 {
    for (i, &cp) in checkpoints.iter().enumerate() {
        if current == cp {
            visited |= 1 << i;
        }
    }

    if current == "out" {
        return if visited == (1 << checkpoints.len()) - 1 {
            1
        } else {
            0
        };
    }

    let key = (current.to_string(), visited);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    let count = graph
        .get(current)
        .unwrap()
        .iter()
        .map(|child| count_paths(graph, child, checkpoints, visited, cache))
        .sum();

    cache.insert(key, count);
    count
}

#[aoc(day11, part1)]
fn part1(input: &HashMap<String, Vec<String>>) -> u64 {
    count_paths(input, "you", &[], 0, &mut HashMap::new())
}

#[aoc(day11, part2)]
fn part2(input: &HashMap<String, Vec<String>>) -> u64 {
    count_paths(input, "svr", &["dac", "fft"], 0, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#
            )),
            5
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#
            )),
            2
        );
    }
}
