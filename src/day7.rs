use std::collections::HashSet;

use anyhow::Error;

pub fn selector(input: String, challenge: u32) -> Result<i64, Error> {
    match challenge {
        1 => part1(input),
        2 => part2(input),
        _ => unreachable!(),
    }
}
//Part 1: Counting based on node
fn part1(input: String) -> Result<i64, Error> {
    let mut count: i64 = 0;
    let mut lines = input.lines();

    let first_line = lines.next().expect("input not empty");
    let area_size = first_line.len();
    let mut beams: HashSet<usize> = HashSet::with_capacity(area_size);
    beams.insert(first_line.find("S").expect("first line has source"));

    let mut beam_contact: HashSet<usize> = HashSet::new();
    for line in lines {
        let line: Vec<u8> = line.bytes().collect();
        //upon further thinking, this cycle is going to be a tad more complicated to get right than
        //expected.
        // Need a separate pair of maps in order to satisfy the borrow checker, and it does avoid a
        // potential complication inside the initial loop.
        // pair of maps was unneeded, only a single contact map.
        for beam in beams.iter() {
            if line[*beam] == b'^' {
                count += 1;
                beam_contact.insert(*beam);
            }
        }
        for beam in beam_contact.iter() {
            beams.insert(*beam - 1);
            beams.insert(*beam + 1);

            beams.remove(beam);
        }
        beam_contact.clear();
    }

    Ok(count)
}

fn part2(input: String) -> Result<i64, Error> {
    //Interesing. This one wound up taking the form of a reccurrence, which made it relatively
    //simple to progam once I got my head around the fact that I did not want a singular sum for
    //this one.
    let mut lines = input.lines();

    let first_line = lines.next().expect("input not empty");
    let area_size = first_line.len();
    let source = first_line.find("S").expect("first line has source");
    let mut beams: Vec<i64> = vec![0; area_size];
    beams[source] = 1;

    let mut beam_contact: HashSet<usize> = HashSet::new();
    for line in lines {
        let line: Vec<u8> = line.bytes().collect();
        for (index, beam) in beams.iter().enumerate() {
            if *beam != 0 && line[index] == b'^' {
                beam_contact.insert(index);
            }
        }
        for beam in beam_contact.iter() {
            let left_branch = *beam - 1;
            beams[left_branch] += beams[*beam];

            let right_branch = *beam + 1;
            beams[right_branch] += beams[*beam];

            beams[*beam] = 0;
        }
        beam_contact.clear();
    }

    Ok(beams.iter().sum())
}

//Part 2: counting the edges
#[cfg(test)]
mod test {
    use super::*;
    fn test_func(challenge: u32) -> i64 {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .to_string();

        selector(input, challenge).unwrap()
    }

    #[test]
    fn part1_provided_input() {
        let expected_result = 21;
        let actual_result = test_func(1);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn part2_provided_input() {
        let expected_result = 40;
        let actual_result = test_func(2);

        assert_eq!(actual_result, expected_result);
    }
}
