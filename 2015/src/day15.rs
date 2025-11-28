use std::{
    iter::Sum,
    ops::{Add, Mul},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Mul<i64> for Ingredient {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Ingredient {
            name: self.name + &format!("*{rhs}"),
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl Add for Ingredient {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Ingredient {
            name: self.name + "+" + &rhs.name,
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Sum for Ingredient {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Ingredient::new(), |acc, i| acc + i)
    }
}

impl Ingredient {
    pub fn new() -> Self {
        Ingredient {
            name: String::new(),
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }

    pub fn score(&self) -> i64 {
        if self.capacity < 0 || self.durability < 0 || self.flavor < 0 || self.texture < 0 {
            return 0;
        }

        self.capacity * self.durability * self.flavor * self.texture
    }
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<Ingredient> {
    let mut ingredients = vec![];
    let re = regex::Regex::new(r"(?P<name>\w+): capacity (?P<capacity>[-\d]+), durability (?P<durability>[-\d]+), flavor (?P<flavor>[-\d]+), texture (?P<texture>[-\d]+), calories (?P<calories>[-\d]+)").unwrap();

    for line in input.lines() {
        let data = re.captures(line).unwrap();
        ingredients.push(Ingredient {
            name: data["name"].to_string(),
            capacity: data["capacity"].to_string().parse().unwrap(),
            durability: data["durability"].to_string().parse().unwrap(),
            flavor: data["flavor"].to_string().parse().unwrap(),
            texture: data["texture"].to_string().parse().unwrap(),
            calories: data["calories"].to_string().parse().unwrap(),
        })
    }
    ingredients
}

#[aoc(day15, part1)]
fn part1(input: &[Ingredient]) -> i64 {
    let mut max = 0;
    let mut factors: Vec<i64> = input.iter().map(|_| 0).collect();
    for x in 1..(1 + 100i64.pow(input.len() as u32)) {
        for (i, v) in factors.iter_mut().enumerate() {
            *v = (x / 100i64.pow(i as u32)) % 100;
        }
        if factors.iter().sum::<i64>() != 100 {
            continue;
        }
        let result = factors
            .iter()
            .enumerate()
            .map(|(i, v)| input[i].clone() * *v)
            .sum::<Ingredient>()
            .score();

        if result > max {
            max = result
        }
    }
    max
}

#[aoc(day15, part2)]
fn part2(input: &[Ingredient]) -> i64 {
    let mut max = 0;
    let mut factors: Vec<i64> = input.iter().map(|_| 0).collect();
    for x in 1..(1 + 100i64.pow(input.len() as u32)) {
        for (i, v) in factors.iter_mut().enumerate() {
            *v = (x / 100i64.pow(i as u32)) % 100;
        }
        if factors.iter().sum::<i64>() != 100 {
            continue;
        }
        let result = factors
            .iter()
            .enumerate()
            .map(|(i, v)| input[i].clone() * *v)
            .sum::<Ingredient>();

        if result.calories == 500 && result.score() > max {
            max = result.score()
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
                   Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#
            )),
            62842880
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
                   Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#
            )),
            57600000
        );
    }
}
