use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
};

use anyhow::{Error, bail};
pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    let (depth, top_n) = match challenge {
        1 => (1000, 3),
        2 => todo!(),
        _ => unreachable!(),
    };
    day(input, depth, top_n)
}
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }
    fn get_distance(&self, p2: &Point) -> f64 {
        let x_dist = ((self.x - p2.x) as f64).powi(2);
        let y_dist = ((self.y - p2.y) as f64).powi(2);
        let z_dist = ((self.z - p2.z) as f64).powi(2);

        (x_dist + y_dist + z_dist).sqrt()
    }
}
#[derive(Debug)]
struct DistancePair<'a> {
    distance: f64,
    p1: &'a Point,
    p2: &'a Point,
}

impl<'a> DistancePair<'a> {
    fn new(distance: f64, p1: &'a Point, p2: &'a Point) -> DistancePair<'a> {
        DistancePair { distance, p1, p2 }
    }
}

impl<'a> PartialEq for DistancePair<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl<'a> Eq for DistancePair<'a> {}
impl<'a> Ord for DistancePair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance).reverse()
    }
}
impl<'a> PartialOrd for DistancePair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn day(input: String, depth: u32, top_n: usize) -> Result<i64, Error> {
    let points: Vec<Point> = parse(input)?;
    let mut distances: BinaryHeap<DistancePair> = BinaryHeap::new();
    //Go through all the points, calculating the distances between them. Push to a Min heap to
    //acquire a sorted selection of distance pairs
    for (index, p1) in points.iter().enumerate() {
        //skip to avoid already checked pairs and to avoid checking against self.
        for p2 in points.iter().skip(index + 1) {
            let distance = p1.get_distance(p2);
            let distance_pair = DistancePair::new(distance, p1, p2);
            distances.push(distance_pair);
        }
    }
    //Assemble the circuits. but how do I do it? Do I simply create a vec of sets of points, and
    //append and merge as needed?
    let mut circuits: Vec<HashSet<&Point>> = Vec::new();
    for _ in 0..depth {
        match distances.pop() {
            Some(d) => {
                let mut circuit_connected: (Option<HashSet<&Point>>, Option<HashSet<&Point>>) =
                    (None, None);
                for circuit in circuits.iter_mut() {
                    if circuit.contains(d.p1) {
                        circuit_connected.0 = Some(std::mem::take(circuit));
                    }
                    if circuit.contains(d.p2) {
                        circuit_connected.1 = Some(std::mem::take(circuit));
                    }
                }
                circuits.retain(|v| !v.is_empty());
                let circuit_to_add = match circuit_connected {
                    (None, None) => {
                        let mut new_circuit = HashSet::new();
                        new_circuit.insert(d.p1);
                        new_circuit.insert(d.p2);
                        new_circuit
                    }
                    (Some(mut circuit), None) => {
                        circuit.insert(d.p2);
                        circuit
                    }
                    (None, Some(mut circuit)) => {
                        circuit.insert(d.p1);
                        circuit
                    }
                    (Some(mut circuit1), Some(circuit2)) => {
                        circuit1.extend(circuit2);
                        circuit1
                    }
                };
                circuits.push(circuit_to_add);
            }
            None => bail!("insufficent points for given depth!"),
        }
    }
    circuits.sort_by_key(|c| Reverse(c.len()));
    let result: usize = circuits
        .into_iter()
        .take(top_n)
        .map(|c| c.len())
        .collect::<Vec<usize>>()
        .into_iter()
        .reduce(|acc, len| acc * len)
        .expect("Can not be empty");

    Ok(result as i64)
}

fn parse(input: String) -> Result<Vec<Point>, Error> {
    let mut points: Vec<Point> = Vec::new();
    let mut coords = Vec::with_capacity(3);
    for line in input.lines() {
        coords = line
            .splitn(3, ',')
            .map(|n| n.parse::<i64>().expect("valid integer"))
            .collect();
        points.push(Point::new(coords[0], coords[1], coords[2]));
    }
    Ok(points)
}
#[cfg(test)]
mod test {
    use super::*;
    fn test_func(_challenge: u32) -> i64 {
        let input = "162,817,812
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
425,690,689"
            .to_string();
        day(input, 10, 3).unwrap()
    }

    #[test]
    fn advent_provided_input1() {
        let expected_result = 40;
        let actual_result = test_func(1);

        assert_eq!(actual_result, expected_result);
    }
}
