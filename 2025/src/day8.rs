use foldhash::{HashMap, HashMapExt, HashSet, HashSetExt};
use std::{cell::Cell, collections::BinaryHeap, ops::Mul, rc::Rc};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default)]
struct Link {
    distance_sq: u64,
    start: Rc<JunctionBox>,
    end: Rc<JunctionBox>,
}

impl Eq for Link {}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.distance_sq.eq(&other.distance_sq)
    }
}

impl Ord for Link {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap() // Floating point distance is totally ordered for this one
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance_sq.partial_cmp(&self.distance_sq)
    }
}

impl Link {
    fn new(start: &Rc<JunctionBox>, end: &Rc<JunctionBox>) -> Link {
        Link {
            distance_sq: start.as_ref() * end.as_ref(),
            start: start.clone(),
            end: end.clone(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn new(x: u64, y: u64, z: u64) -> Self {
        JunctionBox { x, y, z }
    }
}

impl Mul for &JunctionBox {
    type Output = u64;

    fn mul(self, rhs: Self) -> Self::Output {
        let xdiff = self.x.abs_diff(rhs.x);
        let ydiff = self.y.abs_diff(rhs.y);
        let zdiff = self.z.abs_diff(rhs.z);

        xdiff * xdiff + ydiff * ydiff + zdiff * zdiff
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> (Cell<BinaryHeap<Link>>, usize) {
    let mut links: Vec<Link> = vec![];
    let mut junction_boxes = vec![];

    for line in input.lines() {
        let v: Vec<_> = line.split(',').map(|v| v.parse::<u64>().unwrap()).collect();
        let new = &Rc::new(JunctionBox::new(v[0], v[1], v[2]));

        for jb in junction_boxes.iter() {
            links.push(Link::new(jb, new))
        }

        junction_boxes.push(new.clone())
    }

    let mut heap = BinaryHeap::with_capacity(links.len());
    for jb in links {
        heap.push(jb)
    }

    (Cell::new(heap), junction_boxes.len())
}

#[aoc(day8, part1)]
fn part1((links, _): &(Cell<BinaryHeap<Link>>, usize)) -> u64 {
    part1_with_connections(&mut links.take(), 1000)
}

fn part1_with_connections(links: &mut BinaryHeap<Link>, connections: usize) -> u64 {
    let mut junction_boxes: HashMap<Rc<JunctionBox>, Vec<Rc<JunctionBox>>> = HashMap::new();

    for _ in 0..connections {
        let link = links.pop().unwrap();
        let start = link.start.clone();
        let end = link.end.clone();

        junction_boxes
            .entry(start.clone())
            .and_modify(|v| v.push(end.clone()))
            .or_insert(Vec::from([end.clone()]));

        junction_boxes
            .entry(end)
            .and_modify(|v| v.push(start.clone()))
            .or_insert(Vec::from([start]));
    }

    let mut circuits = Vec::new();
    while !junction_boxes.is_empty() {
        let (jb, links) = junction_boxes.iter().next().unwrap();
        let (jb, mut links) = (jb.clone(), links.clone());

        let mut circuit = HashSet::new();
        circuit.insert(jb.clone());
        while let Some(next) = links.pop() {
            circuit.insert(next.clone());
            if junction_boxes.contains_key(&next) {
                links.append(junction_boxes.get_mut(&next).unwrap());
            }
            junction_boxes.remove(&next);
        }

        junction_boxes.remove(&jb);
        circuits.push(circuit.len() as u64);
    }

    circuits.sort_by(|x, y| y.cmp(x)); // Descending order
    circuits[0..3].iter().product()
}

trait Merger {
    fn merge(&mut self);
}

impl Merger for Vec<HashSet<Rc<JunctionBox>>> {
    fn merge(&mut self) {
        let mut merged = Vec::with_capacity(self.len());
        let mut visited = HashSet::new();

        for h in 0..self.len() {
            let mut current = self[h].clone();
            for j in h + 1..self.len() {
                if !self[j].is_disjoint(&self[h]) {
                    current = self[h].union(&self[j]).cloned().collect();
                    visited.insert(j);
                }
            }

            if !visited.contains(&h) {
                merged.push(current);
            }
        }

        std::mem::swap(self, &mut merged);
    }
}

#[aoc(day8, part2)]
fn part2((links, junction_boxes_count): &(Cell<BinaryHeap<Link>>, usize)) -> u64 {
    let mut links = links.take();
    let mut circuits: Vec<HashSet<Rc<JunctionBox>>> = Vec::new();

    let mut last_link: Link = Link::default();
    let mut size = 0;

    while size < *junction_boxes_count {
        let current = links.pop().unwrap();

        let mut exists = false;
        circuits
            .iter_mut()
            .filter(|circuit| circuit.contains(&current.start) || circuit.contains(&current.end))
            .for_each(|circuit| {
                circuit.insert(current.start.clone());
                circuit.insert(current.end.clone());

                exists = true;
            });

        if !exists {
            let mut h = HashSet::new();
            h.insert(current.start.clone());
            h.insert(current.end.clone());
            circuits.push(h);
        } else {
            circuits.merge();
        }

        size = circuits[0].len();
        last_link = current;
    }

    last_link.start.x * last_link.end.x
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
        assert_eq!(part1_with_connections(&mut parse(INPUT).0.take(), 10), 40);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 25272);
    }
}
