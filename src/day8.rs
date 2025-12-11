use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
};

use anyhow::{Error, bail};
pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    match challenge {
        1 => part1(input, 1000, 3),
        2 => part2(input),
        _ => unreachable!(),
    }
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
//Could make some significant improvements to this one. runs in 30ms per run, which likely means
//that my deviation from the standard Kruzkal algorithm implementation

fn part1(input: String, depth: u32, top_n: usize) -> Result<i64, Error> {
    let points = parse(input)?;
    let mut distances: BinaryHeap<DistancePair> = get_distances(&points);
    let mut circuits = make_circuits(&points);
    //Assemble the circuits. but how do I do it? Do I simply create a vec of sets of points, and
    //append and merge as needed?
    for _ in 0..depth {
        circuits = match distances.pop() {
            Some(d) => update_circuits(circuits, d),
            None => bail!("insufficent points for given depth!"),
        }
    }
    circuits.sort_by_key(|c| Reverse(c.len()));
    let result: usize = circuits
        .into_iter()
        .take(top_n)
        .map(|c| c.len())
        .reduce(|acc, len| acc * len)
        .expect("Can not be empty");

    Ok(result as i64)
}

fn part2(input: String) -> Result<i64, Error> {
    let points = parse(input)?;
    let mut distances = get_distances(&points);
    let mut circuits = make_circuits(&points);
    let mut prev_two_junctions_x: [i64; 2] = [0; 2];

    while let Some(d) = distances.pop() {
        prev_two_junctions_x[0] = d.p1.x;
        prev_two_junctions_x[1] = d.p2.x;
        circuits = update_circuits(circuits, d);
        if circuits.len() == 1 {
            break;
        }
    }
    println!("{:?}", prev_two_junctions_x);
    let result = prev_two_junctions_x[0] * prev_two_junctions_x[1];
    Ok(result)
}

fn make_circuits<'a>(points: &'a [Point]) -> Vec<HashSet<&'a Point>> {
    let mut circuits = Vec::new();
    for point in points {
        let mut circuit = HashSet::new();
        circuit.insert(point);
        circuits.push(circuit);
    }
    circuits
}

fn update_circuits<'a>(
    mut circuits: Vec<HashSet<&'a Point>>,
    d: DistancePair<'a>,
) -> Vec<HashSet<&'a Point>> {
    // There's a better way to do this that I should have recognized: Union Find
    let mut circuit_connected: (Option<HashSet<&Point>>, Option<HashSet<&Point>>) = (None, None);
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
        (Some(mut circuit1), Some(circuit2)) => {
            circuit1.extend(circuit2);
            circuit1
        }
        //circuit connects to itself.
        (Some(circuit), None) => circuit,
        _ => unreachable!(),
    };
    circuits.push(circuit_to_add);
    circuits
}

fn parse(input: String) -> Result<Vec<Point>, Error> {
    //This can be vastly simplified, and create s
    let mut points: Vec<Point> = Vec::new();
    for line in input.lines() {
        let coords: Vec<i64> = line
            .splitn(3, ',')
            .map(|n| n.parse::<i64>().expect("valid integer"))
            .collect();
        points.push(Point::new(coords[0], coords[1], coords[2]));
    }
    Ok(points)
}

fn get_distances<'a>(points: &[Point]) -> BinaryHeap<DistancePair> {
    let mut distances = BinaryHeap::new();
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
    distances
}
#[cfg(test)]
mod test {
    use super::*;
    fn test_func(challenge: u32) -> i64 {
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
        match challenge {
            1 => part1(input, 10, 3).unwrap(),
            2 => part2(input).unwrap(),
            _ => unreachable!(),
        }
    }

    #[test]
    fn advent_provided_input1() {
        let expected_result = 40;
        let actual_result = test_func(1);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn advent_provided_input2() {
        let expected_result = 25272;
        let actual_result = test_func(2);

        assert_eq!(actual_result, expected_result);
    }
}
