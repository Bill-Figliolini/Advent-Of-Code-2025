use anyhow::Error;

pub fn day3_1(input: String) -> Result<i64, Error> {
    //Today's problem doesn't seem too bad.
    // Simple example of a two-pointer solution, where
    // I will need to split into lines, then interate over to
    // find the two highest values in sequence.
    //
    // on Further thought, this isn't even a two-poointer solution.
    // this is double linear search
    let mut sum_of_max_joltages: i64 = 0;
    for bank in input.lines() {
        let mut battery1 = '0';
        let mut battery2 = '0';

        for (position, battery) in bank.chars().enumerate() {
            //need to not do this if battery is the last in the bank
            if battery > battery1 && position != bank.len() - 1 {
                battery1 = battery;
                battery2 = '0';
                continue;
            }
            if battery > battery2 {
                battery2 = battery;
            }
        }
        let battery1: i64 = battery1.to_digit(10).unwrap() as i64;
        let battery2: i64 = battery2.to_digit(10).unwrap() as i64;
        let max_joltage: i64 = battery1 * 10 + battery2;
        sum_of_max_joltages += max_joltage;
    }

    Ok(sum_of_max_joltages)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn advent_provided_solution() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111"
            .to_string();
        let intended_output = 357;
        let actual_output = day3_1(input).unwrap();

        assert_eq!(actual_output, intended_output);
    }
}
