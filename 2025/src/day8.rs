use foldhash::{HashMap, HashMapExt, HashSet, HashSetExt};
use std::{cell::Cell, collections::BinaryHeap, rc::Rc};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Link {
    distance: f64,
    start: Rc<JunctionBox>,
    end: Rc<JunctionBox>,
}

impl Eq for Link {}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl Ord for Link {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Link {
    fn new(start: &Rc<JunctionBox>, end: &Rc<JunctionBox>) -> Link {
        Link {
            distance: f64::sqrt(
                ((end.x - start.x) * (end.x - start.x)
                    + (end.y - start.y) * (end.y - start.y)
                    + (end.z - start.z) * (end.z - start.z)) as f64,
            ),
            start: start.clone(),
            end: end.clone(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Cell<BinaryHeap<Link>> {
    let mut links: Vec<Link> = vec![];
    let mut junction_boxes = vec![];

    for line in input.lines() {
        let v: Vec<_> = line.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
        let new = Rc::new(JunctionBox::new(v[0], v[1], v[2]));

        for jb in junction_boxes.iter() {
            links.push(Link::new(jb, &new))
        }

        junction_boxes.push(new)
    }

    let mut heap = BinaryHeap::new();

    for jb in links {
        heap.push(jb)
    }

    Cell::new(heap)
}

fn solver(input: &Cell<BinaryHeap<Link>>, connections: usize) -> i64 {
    let mut heap = input.take();

    let mut connected: HashMap<JunctionBox, Vec<JunctionBox>> = HashMap::new();
    for _ in 0..connections {
        let next = heap.pop().unwrap();
        let start = (*next.start).clone();
        let end = (*next.end).clone();

        connected
            .entry(start.clone())
            .and_modify(|v| v.push(end.clone()))
            .or_insert(Vec::from([end.clone()]));

        connected
            .entry(end)
            .and_modify(|v| v.push(start.clone()))
            .or_insert(Vec::from([start]));
    }

    let mut counter = Vec::new();
    while !connected.is_empty() {
        let (jb, links) = connected.iter_mut().next().unwrap();
        let mut links = links.clone();
        let jb = jb.clone();

        let mut circuit = HashSet::new();
        circuit.insert(jb.clone());
        while let Some(next) = links.pop() {
            circuit.insert(next.clone());
            if connected.contains_key(&next) {
                links.append(connected.get_mut(&next).unwrap());
            }
            connected.remove(&next);
        }

        connected.remove(&jb);
        counter.push(circuit.len() as i64);
    }

    counter.sort();
    counter.reverse();

    counter[0..3].iter().fold(1, |acc, v| v * acc)
}

#[aoc(day8, part1)]
fn part1(input: &Cell<BinaryHeap<Link>>) -> i64 {
    solver(input, 1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn part1_example() {
        assert_eq!(solver(&parse(INPUT), 10), 40);
    }
}
