use aoc_runner_derive::{aoc, aoc_generator};

struct Reindeer {
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn new(speed: u32, duration: u32, rest: u32) -> Self {
        Reindeer {
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
            data["speed"].to_string().parse().unwrap(),
            data["duration"].to_string().parse().unwrap(),
            data["rest"].to_string().parse().unwrap(),
        ))
    }
    reindeers
}

#[aoc(day14, part1)]
fn part1(input: &[Reindeer]) -> u32 {
    input.iter().map(|r| r.distance_after(2503)).max().unwrap()
}

// #[aoc(day14, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

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

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
