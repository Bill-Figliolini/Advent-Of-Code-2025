use anyhow::Error;
pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    match challenge {
        1 => day6_1(input),
        2 => todo!(),
        _ => unreachable!(),
    }
}

fn day6_1(input: String) -> Result<i64, Error> {
    let mut operators: Vec<fn(i64, i64) -> i64> = Vec::new();

    //This can be vastly simplifieed by working from the bottom up.
    // create a vec of operators from the last line, then simple use the values vec
    // as an accumulator for each line.
    let mut line_iter = input.lines().rev();
    let operator_iter = line_iter
        .next()
        .expect("input not empty")
        .split_whitespace();

    let mut accumulator: Vec<i64> = Vec::new();
    for operator in operator_iter {
        match operator {
            "+" => {
                operators.push(|x: i64, y: i64| x + y);
                accumulator.push(0);
            }
            "*" => {
                operators.push(|x: i64, y: i64| x * y);
                accumulator.push(1);
            }
            _ => panic!("not operator in operator line"),
        }
    }

    let results = line_iter.fold(accumulator, |mut accum: Vec<i64>, line| {
        for (index, num) in line.split_whitespace().enumerate() {
            let num = num.parse::<i64>().expect("input is number");
            accum[index] = operators[index](accum[index], num);
        }
        accum
    });

    Ok(results.iter().sum())
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
