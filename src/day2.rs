use anyhow::Error;
fn day2(input: String, validation_func: fn(i64) -> bool) -> Result<i64, Error> {
    //Misread the initial description at first.
    //Did nto consdier that the units were the parenthesis separated Ranges,
    //rather than the end point values.
    //
    //Step 1:
    let mut sum_of_invalid_ids = 0;
    for id_range in input.trim().split(',') {
        let mut ids = id_range.split('-');
        let first_id = ids.next().unwrap();
        let first_id = first_id.parse::<i64>().unwrap();
        let last_id = ids.next().unwrap();
        let last_id = last_id.parse::<i64>().unwrap();

        // Do I really need to be iterating across the whole range?
        // Is there some underlying mathmatical pattern I can use
        // to reduce this to one or two expressions?

        for i in first_id..=last_id {
            if validation_func(i) {
                sum_of_invalid_ids += i;
            }
        }
    }

    Ok(sum_of_invalid_ids)
}
pub fn day2_1(input: String) -> Result<i64, Error> {
    day2(input, check_validity_1)
}
pub fn day2_2(input: String) -> Result<i64, Error> {
    day2(input, check_validity_2)
}

fn check_validity_1(number: i64) -> bool {
    //naive approach to detemining validity.
    //but how do I detemine if an ID is invalid?
    //my initial impression of %11 == 0 is false according
    //to the provided example, it fails on 1010 and 6464.
    //
    //Found an OEIS entry - A020338 - Doublets
    //Formula for generating -> x = n*10^(floor(log10(n))+1) + n
    //floor(log10(n)+1) is a known quantity, it is floor(log10(x)+1)/2
    //Formula for checking -> n = x-x%10^(floor(log10(x)+1)/2)
    let original_number_size = ((number as f64).log10().floor() as u32 + 1) / 2;
    let potential_original_number = number % 10i64.checked_pow(original_number_size).unwrap();

    let result = potential_original_number
        + potential_original_number * 10i64.checked_pow(original_number_size).unwrap();
    number == result
}

fn check_validity_2(number: i64) -> bool {
    //This one seems harder, as we need to check a multitude of potential
    //repetitions, instead of the previous single repition.
    //The naive case for this one would be to convert to a string and use typical
    // Sliding Window style algorithms, and that would necessitate converting back to a string.
    //
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn provided_input_1() {
        let input =  "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        let intended_output = 1227775554;
        let actual_output = day2_1(input);

        assert_eq!(actual_output.unwrap(), intended_output);
    }
    #[test]
    fn provided_input_2() {
        let input =  "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        let intended_output = 4174379265;
        let actual_output = day2_2(input);

        assert_eq!(actual_output.unwrap(), intended_output);
    }

    #[test]
    fn known_invalid_ids() {
        let inputs = vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111, 1212, 6464];
        for input in inputs {
            assert!(check_validity_1(input))
        }
    }
}
