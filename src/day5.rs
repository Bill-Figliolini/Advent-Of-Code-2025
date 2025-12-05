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
    let range = build_id_range(ranges);
    let ingredient_ids = ingredient_ids
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut fresh_count = 0;
    for id in ingredient_ids {
        if range.contains(&id) {
            fresh_count += 1;
        }
    }
    Ok(fresh_count)
}

pub fn day5_2(input: String) -> Result<i64, Error> {
    // Only changes here from part 1 are the removal of ingredient id handling.
    // Which could be donw in a less duplicated way by passing out two values from a function,
    // but that would waste time processing unneeded information.
    //
    // The rest of the changes will be in build_id_range, deduplicating ids.
    let mut ranges: Vec<&str> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        ranges.push(line);
    }

    let range = build_id_range(ranges);

    Ok(range.len() as i64)
}

// Going to need some time to think regarding how to construct the set of id ranges as a type.
// Using a vec seems like an awful idea, but what about a tree perhaps?
//
//  And one case if there are no Overlaps
//  No Overlaps with any -> add new
//  Do I even need to
fn build_id_range(input: Vec<&str>) -> BTreeSet<i64> {
    let mut id_tree: BTreeSet<i64> = BTreeSet::new();

    for range in input {
        let mut nums = range.split('-');
        let range_start = nums.next().unwrap().parse::<i64>().unwrap();
        let range_end = nums.next().unwrap().parse::<i64>().unwrap();
        for id in range_start..=range_end {
            id_tree.insert(id);
        }
    }
    // this was an attempt at premature optimization.
    // Though coming backing and figuring out deduplication here would be nice
    //discovery of overlaps and removal of overlaps
    //
    // AS it turns out, this is what I need to do for part 2.
    // At least to avoid a naive approach of counting all the unique numbers in a hashtable
    // Or perhaps, I could use a tree, and change the algorithm a bit.

    id_tree
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
    #[test]
    fn advent_provided_2() {
        let expected_result = 14;
        let actual_result = test_func(day5_2);

        assert_eq!(actual_result, expected_result);
    }
}
