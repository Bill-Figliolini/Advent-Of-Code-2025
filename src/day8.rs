use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use anyhow::Error;
pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    let (depth, top_n) = match challenge {
        1 => (1000, 5),
        2 => todo!(),
        _ => unreachable!(),
    };
    day(input, depth, top_n)
}
#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn new(x: u64, y: u64, z: u64) -> Point {
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

impl<'a> PartialEq for DistancePair<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl<'a> Ord for DistancePair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(other.distance)
    }
}
impl<'a> PartialOrd for DistancePair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other));
    }
}

fn day(input: String, depth: u32, top_n: u32) -> Result<i64, Error> {
    let points: Vec<Point> = parse(input)?;
    let distances: BinaryHeap<DistancePair>;
    //Go through all the points, calculating the distances between them. Push to a Min heap. Then
    //start pulling and assembling circuits
    for (curr_index, p1) in points.iter().enumerate().take(points.len() / 2) {
        for p2 in points.iter().skip(curr_index) {
            let distance = p1.get_distance(p2);
            distances.push(Reverse((distance, (p1, p2))));
        }
    }
    Ok(0)
}

fn parse(input: String) -> Result<Vec<Point>, Error> {
    let mut points: Vec<Point> = Vec::new();
    let mut coords = Vec::with_capacity(3);
    for line in input.lines() {
        coords = line
            .splitn(3, ',')
            .map(|n| n.parse::<u64>().expect("valid integer"))
            .collect();
        points.push(Point::new(coords[0], coords[1], coords[2]));
    }
    Ok(points)
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
        selector(input, challenge).unwrap()
    }

    #[test]
    fn advent_provided_input1() {
        let expected_result = 40;
        let actual_result = test_func(1);

        assert_eq!(actual_result, expected_result);
    }
}
