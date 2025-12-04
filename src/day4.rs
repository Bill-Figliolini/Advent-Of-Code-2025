use anyhow::Error;
pub fn day4_1(input: String) -> Result<i64, Error> {
    Ok(day4(input))
}

pub fn day4_2(input: String) -> Result<i64, Error> {
    todo!()
}

fn day4(input: String) -> i64 {
    let mut num_of_accessable_rolls = 0;
    // First step, determine the width of a line on the number of lines for boundary checking
    let num_columns: usize = input.chars().take_while(|c| !c.is_whitespace()).count();
    let num_rows: usize = input.lines().count();

    //for ease of checking, conver input into a vec of chars
    let rows = input.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();

    for (current_row, row) in rows.iter().enumerate() {
        let mut accessable_rolls_in_row = 0;
        for (current_column, current) in row.iter().enumerate() {
            //determine if the roll is bound by 4 or more other rolls
            if current == &b'@' {
                let max_row = match current_row {
                    i if i == num_rows => num_rows,
                    i => i + 2,
                };
                let min_row = match current_row {
                    0 => 0,
                    i => i - 1,
                };

                let max_column = match current_column {
                    i if i == num_columns => num_columns,
                    i => i + 2,
                };
                let min_column = match current_column {
                    0 => 0,
                    i => i - 1,
                };

                let mut num_surrounding = 0;
                // found the problem, the current iteration is not going past the current one
                // this is likely because of take being less then rather than less then or equal to
                //now, to do a search over the string to determine the position's neighbors
                for (r, adj_row) in rows.iter().enumerate().take(max_row).skip(min_row) {
                    for (c, position) in
                        adj_row.iter().enumerate().take(max_column).skip(min_column)
                    {
                        if position == &b'@' {
                            if r == current_row && c == current_column {
                                continue;
                            }
                            num_surrounding += 1;
                        }
                    }
                }

                if num_surrounding < 4 {
                    accessable_rolls_in_row += 1;
                }
            }
        }
        num_of_accessable_rolls += accessable_rolls_in_row;
    }
    num_of_accessable_rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_func(function_to_test: fn(String) -> Result<i64, Error>) -> i64 {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();
        function_to_test(input).unwrap()
    }
    #[test]
    fn advent_provided_example_1() {
        let expected_output = 13;
        let actual_output = test_func(day4_1);

        assert_eq!(actual_output, expected_output);
    }
    #[test]
    fn advent_provided_example_2() {
        let expected_output = 43;
        let actual_output = test_func(day4_2);

        assert_eq!(actual_output, expected_output);
    }
}
