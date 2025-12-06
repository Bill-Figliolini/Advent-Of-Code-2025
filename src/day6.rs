use anyhow::Error;
pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    match challenge {
        1 => day6_1(input),
        2 => todo!(),
        _ => unreachable!(),
    }
}

fn day6_1(input: String) -> Result<i64, Error> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_func(function_to_test: fn(String) -> Result<i64, Error>) -> i64 {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .to_string();

        function_to_test(input).unwrap()
    }

    #[test]
    fn advent_provided_code() {
        let intended_output = 4277556;
        let actual_output = test_func(day6_1);

        assert_eq!(actual_output, intended_output);
    }
}
