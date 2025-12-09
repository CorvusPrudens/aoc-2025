use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn dist2(&self, other: &Self) -> i64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        x * x + y * y + z * z
    }
}

#[derive(Clone, Copy, Eq)]
struct Pair {
    distance: i64,
    a: usize,
    b: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

pub fn part_one_first_try(input: &str, max_pairs: usize) -> impl Display {
    let (_, pairs) = parse_junction_pairs(input);
    let mut circuits = Vec::<HashSet<usize>>::new();

    for Pair { a, b, .. } in pairs.into_iter().take(max_pairs) {
        let circuit_a = circuits.iter().position(|c| c.contains(&a));
        let circuit_b = circuits.iter().position(|c| c.contains(&b));

        match (circuit_a, circuit_b) {
            (Some(ca), Some(cb)) => {
                if ca != cb {
                    // merge the circuits
                    let last = circuits.remove(ca.max(cb));
                    circuits[ca.min(cb)].extend(last);
                }
            }
            (Some(ca), None) => {
                circuits[ca].insert(b);
            }
            (None, Some(cb)) => {
                circuits[cb].insert(a);
            }
            (None, None) => {
                circuits.push([a, b].into_iter().collect());
            }
        }
    }

    // NOTE: This isn't any faster.
    // Evidently, the time here is dominated by the above.
    // let mut max_three = vec![0; 3];

    // for circuit in circuits {
    //     let length = circuit.len();
    //     let position = max_three.iter().rev().position(|len| length > *len);
    //     match position {
    //         None | Some(0) => {}
    //         Some(p) => {
    //             max_three.insert(2 - p, length);
    //             max_three.pop();
    //         }
    //     }
    // }

    // max_three.into_iter().fold(1, |a, k| a * k)

    circuits.sort_unstable_by_key(|c| Reverse(c.len()));
    circuits
        .iter()
        .take(3)
        .map(|c| c.len())
        .fold(1, |a, k| a * k)
}

#[derive(Default)]
struct Circuits {
    circuits: HashMap<usize, HashSet<usize>>,
    coordinate_mapping: HashMap<usize, usize>,
    id: usize,
}

impl Circuits {
    /// Returns whether the pair modified any circuits
    fn insert_pair(&mut self, Pair { a, b, .. }: Pair) -> bool {
        let circuit_a = self.coordinate_mapping.get(&a).copied();
        let circuit_b = self.coordinate_mapping.get(&b).copied();

        match (circuit_a, circuit_b) {
            (Some(ca), Some(cb)) => {
                if ca != cb {
                    // merge the circuits
                    let remove = ca.max(cb);
                    let extend = ca.min(cb);

                    for coordinate in &self.circuits[&remove] {
                        self.coordinate_mapping.insert(*coordinate, extend);
                    }

                    let remove = self.circuits.remove(&remove).unwrap();
                    self.circuits.get_mut(&extend).unwrap().extend(remove);
                    true
                } else {
                    false
                }
            }
            (Some(ca), None) => {
                self.circuits.get_mut(&ca).unwrap().insert(b);
                self.coordinate_mapping.insert(b, ca);
                true
            }
            (None, Some(cb)) => {
                self.circuits.get_mut(&cb).unwrap().insert(a);
                self.coordinate_mapping.insert(a, cb);
                true
            }
            (None, None) => {
                self.circuits.insert(self.id, [a, b].into_iter().collect());
                self.coordinate_mapping.insert(a, self.id);
                self.coordinate_mapping.insert(b, self.id);
                self.id += 1;

                true
            }
        }
    }
}

pub fn part_one(input: &str, max_pairs: usize) -> impl Display {
    let mut pairs = parse_junction_pairs_heap(input);
    let mut circuits = Circuits::default();

    let mut count = 0;
    while let Some(pair) = pairs.pop() {
        circuits.insert_pair(pair);
        count += 1;
        if count == max_pairs {
            break;
        }
    }

    let mut max_three = Vec::new();

    for circuit in circuits.circuits.values() {
        let length = circuit.len();
        let position = max_three.binary_search_by(|v| length.cmp(v));
        match position {
            Ok(i) => {
                max_three.insert(i, length);
                if max_three.len() > 3 {
                    max_three.pop();
                }
            }
            Err(i) => {
                if i < 3 {
                    max_three.insert(i, length);
                    if max_three.len() > 3 {
                        max_three.pop();
                    }
                }
            }
        }
    }

    max_three.into_iter().fold(1, |a, k| a * k)
}

pub fn part_two(input: &str) -> impl Display {
    let (junctions, pairs) = parse_junction_pairs(input);
    let mut circuits = Circuits::default();

    let mut last_pair = None;
    for pair in pairs {
        if circuits.insert_pair(pair.clone()) {
            last_pair = Some(pair);
        }
    }

    let last_pair = last_pair.unwrap();
    junctions[last_pair.a].x * junctions[last_pair.b].x
}

fn parse_junction_pairs_heap(input: &str) -> BinaryHeap<Pair> {
    let junctions: Vec<_> = input
        .lines()
        .flat_map(|line| {
            let mut coordinates = line.split(',');
            let x = coordinates.next()?.parse().ok()?;
            let y = coordinates.next()?.parse().ok()?;
            let z = coordinates.next()?.parse().ok()?;

            Some(Coordinate { x, y, z })
        })
        .collect();

    // Find all unique connections and sort them.
    let mut pairs = BinaryHeap::new();
    for (i, a) in junctions.iter().enumerate() {
        for (j, b) in junctions.iter().enumerate().skip(i + 1) {
            let distance = a.dist2(b);
            pairs.push(Pair {
                distance,
                a: i,
                b: j,
            });
        }
    }

    pairs
}

fn parse_junction_pairs(input: &str) -> (Vec<Coordinate>, Vec<Pair>) {
    let junctions: Vec<_> = input
        .lines()
        .flat_map(|line| {
            let mut coordinates = line.split(',');
            let x = coordinates.next()?.parse().ok()?;
            let y = coordinates.next()?.parse().ok()?;
            let z = coordinates.next()?.parse().ok()?;

            Some(Coordinate { x, y, z })
        })
        .collect();

    // Find all unique connections and sort them.
    let mut pairs = Vec::with_capacity(junctions.len().pow(2));
    for (i, a) in junctions.iter().enumerate() {
        for (j, b) in junctions.iter().enumerate().skip(i + 1) {
            let distance = a.dist2(b);
            pairs.push(Pair {
                distance,
                a: i,
                b: j,
            });
        }
    }
    pairs.sort_unstable_by_key(|p| p.distance);

    (junctions, pairs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eight() {
        let input = include_str!("../../inputs/eight_test.txt");
        assert_eq!("40", part_one(input, 10).to_string().as_str());
    }

    #[test]
    fn eight2() {
        let input = include_str!("../../inputs/eight_test.txt");
        assert_eq!("25272", part_two(input).to_string().as_str());
    }
}
