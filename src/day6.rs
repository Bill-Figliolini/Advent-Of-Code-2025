use anyhow::Error;
pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    match challenge {
        1 => day6_1(input),
        2 => todo!(),
        _ => unreachable!(),
    }
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

    let results = values
        .iter()
        .fold(accumulator, |mut accum: Vec<i64>, row_of_nums| {
            for (index, num) in row_of_nums.iter().enumerate() {
                accum[index] = operators[index](accum[index], *num);
            }
            accum
        });

    Ok(results.iter().sum())
}

fn part1_parse(input: Vec<&str>) -> Vec<Vec<i64>> {
    //line.split_whitespace()
    // .parse::<i64>().expect("input is number")
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().expect("input is all nums"))
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn day6_1(input: String) -> Result<i64, Error> {
    day6(input, part1_parse)
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
