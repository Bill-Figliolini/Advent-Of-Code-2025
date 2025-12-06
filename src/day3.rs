use anyhow::Error;
pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    match challenge {
        1 => day3_1(input),
        2 => day3_2(input),
        _ => unreachable!(),
    }
}
fn day3(input: String, check_func: fn(&str) -> i64) -> Result<i64, Error> {
    //Today's problem doesn't seem too bad.
    // Simple example of a two-pointer solution, where
    // I will need to split into lines, then interate over to
    // find the two highest values in sequence.

    let mut sum_of_max_joltages: i64 = 0;
    for bank in input.lines() {
        sum_of_max_joltages += check_func(bank);
    }

    Ok(sum_of_max_joltages)
}

fn day3_1(input: String) -> Result<i64, Error> {
    day3(input, check_two)
}

fn day3_2(input: String) -> Result<i64, Error> {
    day3(input, check_12)
}

fn check_two(bank: &str) -> i64 {
    check_size(bank, 2)
}

fn check_12(bank: &str) -> i64 {
    check_size(bank, 12)
}

fn check_size(bank: &str, size: usize) -> i64 {
    let mut selected_batteries: Vec<char> = vec!['0'; size];
    //Overall desctription of what must be done:
    //iterate over the bank, checking to see if the current
    //bank battery is higher than any of the selected batteries.
    //if it is, select it and invalidate all batteries after that point.
    'bank: for (bank_position, bank_battery) in bank.chars().enumerate() {
        for (battery_position, selected_battery) in selected_batteries.iter_mut().enumerate() {
            if (bank.len() - bank_position) >= (size - battery_position)
                && bank_battery > *selected_battery
            {
                *selected_battery = bank_battery;
                for invalid_battery in selected_batteries
                    .iter_mut()
                    .take(size)
                    .skip(battery_position + 1)
                {
                    *invalid_battery = '0';
                }
                //batteries can only occur once in the output,
                //so return to the bank after selection.
                continue 'bank;
            }
        }
    }
    let mut max_joltage = 0;
    for battery in selected_batteries {
        //shift right by one power of ten if not the last
        max_joltage *= 10;
        //add battery
        max_joltage += battery.to_digit(10).unwrap() as i64;
    }
    max_joltage
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_func(function_to_test: fn(String) -> Result<i64, Error>) -> i64 {
        let input = "987654321111111
        811111111111119
        234234234234278
        818181911112111"
            .to_string();
        function_to_test(input).unwrap()
    }
    #[test]
    fn advent_provided_solution_1() {
        let intended_output = 357;
        let actual_output = test_func(day3_1);

        assert_eq!(actual_output, intended_output);
    }
    #[test]
    fn advent_provided_solution_2() {
        let intended_output = 3121910778619;
        let actual_output = test_func(day3_2);

        assert_eq!(actual_output, intended_output);
    }

    #[test]
    fn check_size_direct_input() {
        let inputs = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let expected_outputs = [987654321111, 811111111119, 434234234278, 888911112111];
        for i in 0..4 {
            let actual_output = check_12(inputs[i]);
            assert_eq!(actual_output, expected_outputs[i]);
        }
    }
}
