use std::cmp::max;

use anyhow::Error;

pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    match challenge {
        1 => part1(input),
        2 => part2(input),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
    fn area_enclosed(&self, other: &Point) -> i64 {
        let diff_x = (self.x - other.x).abs() + 1;
        let diff_y = (self.y - other.y).abs() + 1;

        diff_x * diff_y
    }
}

fn parse(input: String) -> Result<Vec<Point>, Error> {
    let mut points = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<i64>()?;
        let y = y.parse::<i64>()?;
        points.push(Point::new(x, y));
    }

    Ok(points)
}

fn part1(input: String) -> Result<i64, Error> {
    let points = parse(input)?;
    let mut maximum_area = 0;

    for (index, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(index + 1) {
            maximum_area = max(maximum_area, point1.area_enclosed(point2));
        }
    }

    Ok(maximum_area)
}

fn part2(input: String) -> Result<i64, Error> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_func(function_to_test: fn(String) -> Result<i64, Error>) -> i64 {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        .to_string();
        function_to_test(input).unwrap()
    }

    #[test]
    fn advent_provided_input_part1() {
        let intended_result = 50;
        let actual_result = test_func(part1);

        assert_eq!(actual_result, intended_result);
    }

    #[test]
    fn advent_provided_input_part2() {
        let intended_result = 24;
        let actual_result = test_func(part2);

        assert_eq!(actual_result, intended_result);
    }
}
