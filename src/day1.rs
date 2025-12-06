use anyhow::{Error, bail};

pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    match challenge {
        1 => day1_1(input),
        2 => day1_2(input),
        _ => unreachable!(),
    }
}

fn day1_1(input: String) -> Result<i64, Error> {
    let turns = day1_parser(input)?;
    let result = day1_1_solver(turns);

    Ok(result)
}

fn day1_2(input: String) -> Result<i64, Error> {
    let turns = day1_parser(input)?;
    let result = day1_2_solver(turns);

    Ok(result)
}

fn day1_parser(input: String) -> Result<Vec<i64>, Error> {
    //Split the string into lines, Take the
    //First Character off the front for the direction,
    //Then Convert to Int
    //Lefts are subtractions,
    //Rights are additions
    let mut int_turns = Vec::new();
    for text_turn in input.lines() {
        let mut turn = text_turn.to_string();
        let direction = match turn.remove(0) {
            'L' => -1,
            'R' => 1,
            _ => bail!("Parse Error"),
        };
        let magnitude = turn.parse::<i64>()?;
        int_turns.push(magnitude * direction);
    }
    Ok(int_turns)
}

fn day1_1_solver(turns: Vec<i64>) -> i64 {
    //Addition over a rotating lock is
    //Modular Arithmetic modulo the number of positions
    let mut current_dial_position = 50;
    let mut times_at_zero = 0;

    for turn in turns {
        current_dial_position += turn;
        current_dial_position %= 100;

        if current_dial_position == 0 {
            times_at_zero += 1;
        }
    }

    times_at_zero
}

fn day1_2_solver(turns: Vec<i64>) -> i64 {
    //A tad more complicated,tracking the number of times the dial goes over 100.
    //Modular Arithmetic still needed, but I also need to deal with how the number
    //rolls over 100.
    let mut current_dial_position = 50;
    let mut times_past_zero = 0;

    for turn in turns {
        let crossings: i64 = if turn >= 0 {
            (current_dial_position + turn).div_euclid(100)
        } else {
            (current_dial_position - 1).div_euclid(100)
                - (current_dial_position + turn - 1).div_euclid(100)
        };
        current_dial_position = (current_dial_position + turn).rem_euclid(100);
        times_past_zero += crossings.abs()
    }

    times_past_zero
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn advent_provided_input_for_day1_1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string();
        println!("{}", input);
        let intended_result = 3;
        let actual_result = day1_1(input);
        assert_eq!(actual_result.unwrap(), intended_result)
    }
    #[test]
    fn advent_provided_input_for_day1_2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string();
        println!("{}", input);
        let intended_result = 6;
        let actual_result = day1_2(input);
        assert_eq!(actual_result.unwrap(), intended_result)
    }

    #[test]
    fn multiple_hits_to_0_right() {
        let input = "R1000".to_string();
        let intended_result = 10;
        let actual_result = day1_2(input);

        assert_eq!(actual_result.unwrap(), intended_result)
    }
    #[test]
    fn multiple_hits_to_0_left() {
        let input = "L1000".to_string();
        let intended_result = 10;
        let actual_result = day1_2(input);

        assert_eq!(actual_result.unwrap(), intended_result)
    }
}
