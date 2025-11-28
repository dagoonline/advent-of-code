use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

struct Reindeer {
    name: String,
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn new(name: String, speed: u32, duration: u32, rest: u32) -> Self {
        Reindeer {
            name,
            speed,
            duration,
            rest,
        }
    }

    fn distance_after(&self, time: u32) -> u32 {
        let amount = time / (self.duration + self.rest);
        let remainder = time % (self.duration + self.rest);

        let extra = if remainder > self.duration {
            self.duration * self.speed
        } else {
            remainder * self.speed
        };

        self.speed * self.duration * amount + extra
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Reindeer> {
    let mut reindeers = vec![];
    let re = regex::Regex::new(r"(?P<reindeer>\w+) can fly (?P<speed>\d+) km/s for (?P<duration>\d+) seconds, but then must rest for (?P<rest>\d+) seconds").unwrap();

    for line in input.lines() {
        let data = re.captures(line).unwrap();
        reindeers.push(Reindeer::new(
            data["reindeer"].to_string(),
            data["speed"].to_string().parse().unwrap(),
            data["duration"].to_string().parse().unwrap(),
            data["rest"].to_string().parse().unwrap(),
        ))
    }
    reindeers
}

#[aoc(day14, part1)]
fn part1(reindeers: &[Reindeer]) -> u32 {
    reindeers
        .iter()
        .map(|r| r.distance_after(2503))
        .max()
        .unwrap()
}

#[aoc(day14, part2)]
fn part2(reindeers: &[Reindeer]) -> u32 {
    let mut standings = HashMap::<String, u32>::new();

    for time in 1..2504 {
        let race: Vec<(String, u32)> = reindeers
            .iter()
            .map(|r| (r.name.clone(), r.distance_after(time)))
            .collect();
        let (_, max_distance) = race
            .iter()
            .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
            .unwrap()
            .to_owned();
        let ties: Vec<String> = race
            .into_iter()
            .filter(|(_, v)| *v == max_distance)
            .map(|(r, _)| r)
            .collect();

        for winner in ties {
            standings.entry(winner).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    standings.values().max().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
                Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#
            )),
            2660
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
                Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#
            )),
            1564
        );
    }
}
