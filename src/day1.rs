use anyhow::{Error,bail};

pub fn day1(input: String) -> Result<i64, Error>{
    let turns = day1_parser(input)?;
    let result = day1_solver(turns);

    Ok(result)
}

fn day1_parser(input: String) -> Result<Vec<i64>, Error>{
    //Split the string into lines, Take the
    //First Character off the front for the direction,
    //Then Convert to Int
    //Lefts are subtractions,
    //Rights are additions
    let mut int_turns = Vec::new();
    for text_turn in input.lines(){
        let mut turn = text_turn.to_string();
        let direction = match turn.remove(0){
            'L' => -1,
            'R' => 1,
            _ => bail!("Parse Error")
        };
        let magnitude = turn.parse::<i64>()?;
        int_turns.push(magnitude * direction);
    };
    Ok(int_turns)
}

fn day1_solver(turns: Vec<i64>) -> i64{
    //Addition over a rotating lock is 
    //Modular Arithmetic modulo the number of positions
    let mut current_dial_position = 50;
    let mut times_at_zero = 0;

    for turn in turns{
        current_dial_position += turn;
        current_dial_position %= 100;

        if current_dial_position == 0{
            times_at_zero+=1;
        }
    }

    times_at_zero
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn advent_provided_input(){
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82".to_string();
        println!("{}", input);
        let intended_result = 3;
        let actual_result = day1(input);
        assert_eq!(actual_result.unwrap(), intended_result)

    }
}
