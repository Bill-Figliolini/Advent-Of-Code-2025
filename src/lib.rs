mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
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

    let result = match day {
        1 => day1::selector(file_contents, challenge),
        2 => day2::selector(file_contents, challenge),
        3 => day3::selector(file_contents, challenge),
        4 => day4::selector(file_contents, challenge),
        5 => day5::selector(file_contents, challenge),
        6 => day6::selector(file_contents, challenge),
        7..12 => todo!(),
        _ => unreachable!(),
    };
    match result {
        Ok(result) => println!("Day {day} result is: {result}"),
        Err(e) => println!("Day {day} errored with: {e}"),
    }
}
