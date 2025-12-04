use anyhow::Error;
pub fn day4_1(input: String) -> Result<i64, Error> {
    //for ease of checking, conver input into a vec of chars
    let rows = string_to_2d_vec(input);
    let (result, _) = day4(rows);
    Ok(result)
}

pub fn day4_2(input: String) -> Result<i64, Error> {
    //Interesting potential thought here
    //wework day4 to be generate a new graph and return a tuple with it and the
    // result to handle the repeated iterations of the part 2 challenge
    let mut rows = string_to_2d_vec(input);
    let mut number_moved;
    (number_moved, rows) = day4(rows);

    let mut result = 0;
    result += number_moved;
    while number_moved != 0 {
        (number_moved, rows) = day4(rows);
        result += number_moved;
    }
    Ok(result)
}

fn string_to_2d_vec(input: String) -> Vec<Vec<u8>> {
    input
        .split_whitespace()
        .map(|l| l.to_string().into_bytes())
        .collect::<Vec<Vec<u8>>>()
}

fn day4(rows: Vec<Vec<u8>>) -> (i64, Vec<Vec<u8>>) {
    let mut num_of_accessable_rolls = 0;
    // First step, determine the width of a line on the number of lines for boundary checking
    let num_columns: usize = rows[0].len();
    let num_rows: usize = rows.len();

    let mut next_table: Vec<Vec<u8>> = Vec::with_capacity(num_rows);

    for (current_row, row) in rows.iter().enumerate() {
        let mut accessable_rolls_in_row = 0;
        let mut new_row: Vec<u8> = Vec::with_capacity(num_columns);
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
                    new_row.push(b'.');
                } else {
                    new_row.push(b'@');
                }
            } else {
                new_row.push(b'.');
            }
        }
        num_of_accessable_rolls += accessable_rolls_in_row;
        next_table.push(std::mem::take(&mut new_row));
    }
    (num_of_accessable_rolls, next_table)
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
