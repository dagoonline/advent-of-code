use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, Debug)]
struct Aunt {
    name: u16,
    children: i8,
    cats: i8,
    samoyeds: i8,
    pomeranians: i8,
    akitas: i8,
    vizslas: i8,
    goldfish: i8,
    trees: i8,
    cars: i8,
    perfumes: i8,
}

impl Aunt {
    fn new(name: u16) -> Aunt {
        Aunt {
            name,
            children: -1,
            cats: -1,
            samoyeds: -1,
            pomeranians: -1,
            akitas: -1,
            vizslas: -1,
            goldfish: -1,
            trees: -1,
            cars: -1,
            perfumes: -1,
        }
    }

    fn part1_eq(&self, other: &Self) -> bool {
        (self.children == -1 || self.children == other.children)
            && (self.cats == -1 || self.cats == other.cats)
            && (self.samoyeds == -1 || self.samoyeds == other.samoyeds)
            && (self.pomeranians == -1 || self.pomeranians == other.pomeranians)
            && (self.akitas == -1 || self.akitas == other.akitas)
            && (self.vizslas == -1 || self.vizslas == other.vizslas)
            && (self.goldfish == -1 || self.goldfish == other.goldfish)
            && (self.trees == -1 || self.trees == other.trees)
            && (self.cars == -1 || self.cars == other.cars)
            && (self.perfumes == -1 || self.perfumes == other.perfumes)
    }

    fn part2_eq(&self, other: &Self) -> bool {
        (self.cats == -1 || self.cats > other.cats)
            && (self.trees == -1 || self.trees > other.trees)
            && (self.pomeranians == -1 || self.pomeranians < other.pomeranians)
            && (self.goldfish == -1 || self.goldfish < other.goldfish)
            && (self.children == -1 || self.children == other.children)
            && (self.samoyeds == -1 || self.samoyeds == other.samoyeds)
            && (self.akitas == -1 || self.akitas == other.akitas)
            && (self.vizslas == -1 || self.vizslas == other.vizslas)
            && (self.cars == -1 || self.cars == other.cars)
            && (self.perfumes == -1 || self.perfumes == other.perfumes)
    }
}

impl From<&str> for Aunt {
    fn from(line: &str) -> Self {
        let re = regex::Regex::new(r"Sue (?P<name>\d+): (?P<rest>.+)").unwrap();
        let data = re.captures(line).unwrap();
        let name = data["name"].to_string().parse().unwrap();
        let mut aunt = Aunt::new(name);
        let mut rest = data["rest"].to_string();
        rest.retain(|c| c != ',' && c != ':');
        let rest = rest.split_whitespace().collect::<Vec<&str>>();
        let chunks = rest.chunks(2);
        for compound in chunks {
            let compound_value = compound[1].parse().unwrap();
            match compound[0] {
                "children" => aunt.children = compound_value,
                "cats" => aunt.cats = compound_value,
                "samoyeds" => aunt.samoyeds = compound_value,
                "pomeranians" => aunt.pomeranians = compound_value,
                "akitas" => aunt.akitas = compound_value,
                "vizslas" => aunt.vizslas = compound_value,
                "goldfish" => aunt.goldfish = compound_value,
                "trees" => aunt.trees = compound_value,
                "cars" => aunt.cars = compound_value,
                "perfumes" => aunt.perfumes = compound_value,
                _ => unreachable!(),
            }
        }
        aunt
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Vec<Aunt> {
    let mut aunts = vec![];
    for line in input.lines() {
        aunts.push(Aunt::from(line))
    }
    aunts
}

#[aoc(day16, part1)]
fn part1(input: &Vec<Aunt>) -> u16 {
    let my_aunt = Aunt {
        name: 0,
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };

    let mut latest = 0;
    for aunt in input {
        if aunt.part1_eq(&my_aunt) {
            latest = aunt.name;
        }
    }
    latest
}

#[aoc(day16, part2)]
fn part2(input: &Vec<Aunt>) -> u16 {
    let my_aunt = Aunt {
        name: 0,
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };

    let mut aunts = vec![];
    for aunt in input {
        if aunt.part2_eq(&my_aunt) {
            aunts.push(aunt.name);
        }
    }
    println!("{aunts:?}");
    *aunts.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r#"Sue 1: children: 1, cars: 8, vizslas: 7
                   Sue 2: akitas: 0, perfumes: 1, children: 3"#
            )),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r#"Sue 1: children: 1, cars: 8, vizslas: 7
                   Sue 2: perfumes: 1, cats: 10, pomeranians: 1"#
            )),
            2
        );
    }
}
