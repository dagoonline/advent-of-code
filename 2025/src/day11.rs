use aoc_runner_derive::{aoc, aoc_generator};
use foldhash::{HashMap, HashMapExt};

#[aoc_generator(day11)]
fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (key, rest) = line.split_once(':').unwrap();
        for target in rest.split_whitespace() {
            graph
                .entry(key.to_string())
                .and_modify(|v: &mut Vec<String>| v.push(target.into()))
                .or_insert(vec![target.into()]);
        }
    }

    graph
}

fn paths(input: &HashMap<String, Vec<String>>, key: &str) -> u64 {
    if key == "out" {
        return 1;
    }

    input
        .get(key)
        .unwrap()
        .iter()
        .map(|v| paths(input, v))
        .sum()
}

#[aoc(day11, part1)]
fn part1(input: &HashMap<String, Vec<String>>) -> u64 {
    paths(input, "you")
}

fn paths_fft_dac(
    input: &HashMap<String, Vec<String>>,
    fft: bool,
    dac: bool,
    current: &str,
    cache: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    let dac = dac || current == "dac";
    let fft = fft || current == "fft";
    let key = (current.to_string(), dac, fft);

    if current == "out" {
        return if dac && fft { 1 } else { 0 };
    }

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let count = input
        .get(current)
        .unwrap()
        .iter()
        .map(|v| paths_fft_dac(input, fft, dac, v, cache))
        .sum();

    cache.insert(key, count);

    count
}

#[aoc(day11, part2)]
fn part2(input: &HashMap<String, Vec<String>>) -> u64 {
    let mut cache = HashMap::new();
    paths_fft_dac(input, false, false, "svr", &mut cache)
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
