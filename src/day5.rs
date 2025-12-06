use anyhow::Error;

pub fn day5_1(input: String) -> Result<i64, Error> {
    //first step will be to split the input into its two constitutent parts;
    //The ranges in the starting half, and then ids to check for the second
    let (ranges, ingredients) = input
        .split_once("\n\n")
        .expect("must be split by a double newline");

    let ranges = build_id_range(ranges);
    let ingredient_ids = ingredients
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut fresh_count = 0;
    'next_id: for id in ingredient_ids {
        for (range_start, range_end) in ranges.iter() {
            if *range_start <= id && id <= *range_end {
                fresh_count += 1;
                continue 'next_id;
            }
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
    let (ranges, _) = input
        .split_once("\n\n")
        .expect("must be split by a double newline");

    let range = build_id_range(ranges);
    Ok(range.iter().map(|(start, end)| end - start + 1).sum())
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
//
//  Or we can simplify by reframing, ignoring the left overlaps entirely by sorting in advance and
//  growing from the left.
fn build_id_range(input: &str) -> Vec<(i64, i64)> {
    let mut id_ranges: Vec<(i64, i64)> = input
        .lines()
        .map(|line| line.split_once('-').expect("incorrect Format"))
        .map(|pair| {
            (
                pair.0.parse::<i64>().expect("Not a Num"),
                pair.1.parse::<i64>().expect("Not a Num"),
            )
        })
        .collect();
    id_ranges.sort();
    //clean up pass for any discovered adjacent ranges
    id_ranges
        .iter()
        .fold(Vec::new(), |mut accum: Vec<(i64, i64)>, &curr_range| {
            if let Some(prev_range) = accum.last_mut() {
                if curr_range.0 <= prev_range.1 {
                    prev_range.1 = std::cmp::max(prev_range.1, curr_range.1);
                } else {
                    accum.push(curr_range)
                }
                return accum;
            }
            accum.push(curr_range);
            accum
        })
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
    mod build_id_range {
        use super::*;

        #[test]
        fn handles_overlaps_after() {
            let input = "10-14\n12-18";
            let intended_output = vec![(10, 18)];
            let actual_output = build_id_range(input);

            assert_eq!(actual_output, intended_output);
        }
        #[test]
        fn handles_appends_after() {
            let input = "10-14\n15-18";
            let intended_output = vec![(10, 18)];
            let actual_output = build_id_range(input);

            assert_eq!(actual_output, intended_output);
        }

        #[test]
        fn handles_overlaps_before() {
            let input = "10-14\n5-11";
            let intended_output = vec![(5, 14)];
            let actual_output = build_id_range(input);

            assert_eq!(actual_output, intended_output);
        }
        #[test]
        fn handles_appends_before() {
            let input = "10-14\n5-9";
            let intended_output = vec![(5, 14)];
            let actual_output = build_id_range(input);

            assert_eq!(actual_output, intended_output);
        }
    }
}
