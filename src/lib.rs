mod day1;
mod day2;
use std::fs;

pub fn day_selector(mut arguments: Vec<String>) {
    let day = arguments[1].parse::<u32>();
    let file_read = fs::read_to_string(std::mem::take(&mut arguments[2]));
    let day = match day {
        Ok(day) if day == 0 || day > 12 => {
            println!("Day must be between 1 and 12, inclusive.");
            return;
        }
        Err(e) => {
            println!("Date Conversion Error: {e}");
            return;
        }
        Ok(date) => date,
    };
    let file_contents = match file_read {
        Ok(file_contents) => file_contents,
        Err(e) => {
            println!("File Read Error: {e}");
            return;
        }
    };

    let result = match day {
        1 => day1::day1_2(file_contents),
        2 => day2::day2_1(file_contents),
        3..12 => todo!(),
        _ => unreachable!(),
    };
    match result {
        Ok(result) => println!("Day {day} result is: {result}"),
        Err(e) => println!("Day {day} errored with: {e}"),
    }
}
