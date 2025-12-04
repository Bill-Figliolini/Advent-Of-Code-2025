use Advent_Of_Code_2025::day_selector;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    println!("{:?}", arguments);
    match arguments.len(){
        4 =>day_selector(arguments),
        _ =>{
            println!("Usage: aoc2025 [day#] [challenge#] [input file]");
        }
    }
}


