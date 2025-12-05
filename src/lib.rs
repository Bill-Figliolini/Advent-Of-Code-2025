mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
use std::fs;

pub fn day_selector(mut arguments: Vec<String>) {
    let day = arguments[1].parse::<u32>();
    let challenge = arguments[2].parse::<u32>();
    let file_read = fs::read_to_string(std::mem::take(&mut arguments[3]));
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
    let challenge = match challenge {
        Ok(challenge) if challenge == 1 || challenge == 2 => challenge,
        Ok(_) => {
            println!("Challenge must be 1 or 2");
            return;
        }
        Err(e) => {
            println!("Challenge conversion Error: {e}");
            return;
        }
    };
    let file_contents = match file_read {
        Ok(file_contents) => file_contents,
        Err(e) => {
            println!("File Read Error: {e}");
            return;
        }
    };

    let result = match (day, challenge) {
        (1, 1) => day1::day1_1(file_contents),
        (1, 2) => day1::day1_2(file_contents),
        (2, 1) => day2::day2_1(file_contents),
        (2, 2) => day2::day2_2(file_contents),
        (3, 1) => day3::day3_1(file_contents),
        (3, 2) => day3::day3_2(file_contents),
        (4, 1) => day4::day4_1(file_contents),
        (4, 2) => day4::day4_2(file_contents),
        (5, 1) => day5::day5_1(file_contents),
        (5, 2) => day5::day5_2(file_contents),
        (6..12, _) => todo!(),
        _ => unreachable!(),
    };
    match result {
        Ok(result) => println!("Day {day} result is: {result}"),
        Err(e) => println!("Day {day} errored with: {e}"),
    }
}
