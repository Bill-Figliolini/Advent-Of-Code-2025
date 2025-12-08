use anyhow::Error;
pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    let parse_func = match challenge {
        1 => part1_parse,
        2 => part2_parse,
        _ => unreachable!(),
    };
    day6(input, parse_func)
}

fn day6(input: String, parse_func: fn(Vec<&str>) -> Vec<Vec<i64>>) -> Result<i64, Error> {
    //This can be vastly simplified by working from the bottom up.
    // create a vec of operators from the last line, then simple use the values vec
    // as an accumulator for each line.
    let mut lines: Vec<&str> = input.lines().collect();
    let operator_line = lines.pop().expect("Input Not Empty");

    let mut operators: Vec<fn(i64, i64) -> i64> = Vec::new();
    let mut accumulator: Vec<i64> = Vec::new();
    for operator in operator_line.split_whitespace() {
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
    let values: Vec<Vec<i64>> = parse_func(lines);

    let result = values
        .iter()
        .enumerate()
        .fold(accumulator, |mut accum: Vec<i64>, (index, row_of_nums)| {
            for val in row_of_nums {
                accum[index] = operators[index](accum[index], *val);
            }
            accum
        })
        .iter()
        .sum();

    Ok(result)
}

fn part1_parse(input: Vec<&str>) -> Vec<Vec<i64>> {
    let output = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().expect("input is all nums"))
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    //transpose the output to match the format in part2_parse
    let len = output[0].len();
    let mut iters: Vec<_> = output.into_iter().map(|n| n.into_iter()).collect();
    let output = (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    println!("{:?}", output);
    output
}

fn part2_parse(input: Vec<&str>) -> Vec<Vec<i64>> {
    //turn the input into a 2d vec of bytes, as we know that the input is all ascii
    //and we need to traversie it in two directions
    let input: Vec<Vec<u8>> = input.iter().map(|line| line.bytes().collect()).collect();

    let mut output: Vec<Vec<i64>> = Vec::new();
    let mut nums: Vec<i64> = Vec::new();
    let mut accumulator: Option<i64> = None;
    let num_columns = input[0].len();
    let chars_per_column = input.len();
    for column in 0..num_columns {
        //Needs to read the numbers in a way such that they can be either at the top or bottom
        //of the column. as such, the way I should be working is accumulate into an i64,
        //multiplying by ten for each number added.
        // each vec corresponds to a line of the input, and as such we need to work down the
        // vectors, following the same index until we hit the end.
        for row in 0..chars_per_column {
            let current_char = input[row][column];
            if current_char.is_ascii_digit() {
                accumulator = match accumulator {
                    None => Some((current_char - b'0') as i64),
                    Some(mut current_value) => {
                        current_value *= 10;
                        current_value += (input[row][column] - b'0') as i64;
                        Some(current_value)
                    }
                }
            }
        }
        match accumulator {
            None => output.push(std::mem::take(&mut nums)),
            Some(_) => nums.push(accumulator.take().unwrap()),
        }
    }
    output.push(std::mem::take(&mut nums));
    output
    //Test Errors at present, because I mixed up the order of the arrays. It will be easier to
    //rewrite the other func, so I will.
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_func(function_to_test: fn(Vec<&str>) -> Vec<Vec<i64>>) -> i64 {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .to_string();

        day6(input, function_to_test).unwrap()
    }

    #[test]
    fn advent_provided_code_1() {
        let intended_output = 4277556;
        let actual_output = test_func(part1_parse);
        assert_eq!(actual_output, intended_output);
    }
    #[test]
    fn advent_provided_code_2() {
        let intended_output = 3263827;
        let actual_output = test_func(part2_parse);

        assert_eq!(actual_output, intended_output);
    }
}
