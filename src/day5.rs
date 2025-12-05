use std::collections::BTreeSet;

use anyhow::Error;

pub fn day5_1(input: String) -> Result<i64, Error> {
    //first step will be to split the input into its two constitutent parts;
    //The ranges in the starting half, and then ids to check for the second
    let mut ranges: Vec<&str> = Vec::new();
    let mut ingredient_ids: Vec<&str> = Vec::new();

    {
        let mut recieving_vec = &mut ranges;
        for line in input.lines() {
            if line.is_empty() {
                recieving_vec = &mut ingredient_ids;
            } else {
                (*recieving_vec).push(line);
            }
        }
    }
    let ranges = build_id_range(ranges);
    let ingredient_ids = ingredient_ids
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut expired_counter = 0;
    'next_id: for id in ingredient_ids {
        for (range_start, range_end) in ranges.iter() {
            if *range_start <= id && id <= *range_end {
                continue 'next_id;
            }
        }
        expired_counter += 1;
    }
    Ok(expired_counter)
}
// Going to need some time to think regarding how to construct the set of id ranges as a type.
// Using a vec seems like an awful idea, but what about a tree perhaps?
//
// There are 4 different cases that need to be handled here:
//  Left overlap -> range_start+1 <= new_range_end -> range_start = new_range_start
//  Right overlap -> range_end+1 <= new_range_start -> range_end = new_range_end
//  New total overlap -> range_start <= new_range_end && range_end <= new_range_start -> replace both
//  Old total overlap -> range_start <= new_range_start && new_range_end <= range_end -> do not add
//  new
//  And one case if there are no Overlaps
//  No Overlaps with any -> add new
fn build_id_range(input: Vec<&str>) -> Vec<(i64, i64)> {
    let mut id_ranges: Vec<(i64, i64)> = Vec::new();

    'next_range: for range in input {
        let mut nums = range.split('-');
        let new_range_start = nums.next().unwrap().parse::<i64>().unwrap();
        let new_range_end = nums.next().unwrap().parse::<i64>().unwrap();

        for (range_start, range_end) in id_ranges.iter_mut() {
            let mut found = false;
            if (*range_end) <= new_range_start {
                *range_end = new_range_end;
                found = true;
            }
            if new_range_end <= (*range_start) {
                *range_start = new_range_start;
                found = true;
            }
            if *range_start <= new_range_start && new_range_end <= *range_end {
                found = true;
            }
            if found {
                continue 'next_range;
            }
        }
        //not in ranges
        id_ranges.push((new_range_start, new_range_end));
    }
    id_ranges.sort();
    //clean up pass for any discovered adjacent ranges
    id_ranges
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_func(func_to_test: fn(String) -> Result<i64, Error>) -> i64 {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        .to_string();
        func_to_test(input).unwrap()
    }

    #[test]
    fn advent_provided_1() {
        let exptected_result = 3;
        let actual_result = test_func(day5_1);

        assert_eq!(actual_result, exptected_result);
    }

    mod build_id_range {
        use super::*;

        #[test]
        fn handles_overlaps_after() {
            let input = vec!["10-14", "12-18"];
            let intended_output = vec![(10, 18)];
            let actual_output = build_id_range(input);

            assert_eq!(actual_output, intended_output);
        }
        #[test]
        fn handles_appends_after() {
            let input = vec!["10-14", "15-18"];
            let intended_output = vec![(10, 18)];
            let actual_output = build_id_range(input);

            assert_eq!(actual_output, intended_output);
        }

        #[test]
        fn handles_overlaps_before() {
            let input = vec!["10-14", "5-11"];
            let intended_output = vec![(5, 14)];
            let actual_output = build_id_range(input);

            assert_eq!(actual_output, intended_output);
        }
        #[test]
        fn handles_appends_before() {
            let input = vec!["10-14", "5-9"];
            let intended_output = vec![(5, 14)];
            let actual_output = build_id_range(input);

            assert_eq!(actual_output, intended_output);
        }
    }
}
