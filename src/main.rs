mod puzzle;

fn main() {
    // get first passed argument to cli for day selection
    let Some(day_string) = std::env::args()
        .nth(1) else {
            println!("First argument needs to be day");
            return
        };

    let Ok(day) = day_string.parse::<u8>() else {
        println!("First argument needs to be day as number");
        return
    };

    // get secound passed argument to cli for file path
    let Some(path) = std::env::args()
        .nth(2) else {
            println!("Secound argument needs to be the path to the challenge");
            return
        };

    let Ok(content) = std::fs::read_to_string(&path) else {
        println!("Can not read from file \"{path}\"");
        return
    };

    match day {
        1 => {
            println!("Result part 1: {}", puzzle::day1::part1(&content));
            println!("Result part 2: {}", puzzle::day1::part2(&content));
        }
        2 => {
            println!("Result part 1: {}", puzzle::day2::part1(&content));
            println!("Result part 2: {}", puzzle::day2::part2(&content));
        }
        3 => {
            println!("Result part 1: {}", puzzle::day3::part1(&content));
            println!("Result part 2: {}", puzzle::day3::part2(&content));
        }
        4 => {
            println!("Result part 1: {}", puzzle::day4::part1(&content));
            println!("Result part 2: {}", puzzle::day4::part2(&content));
        }
        5 => {
            println!("Result part 1: {}", puzzle::day5::part1(&content));
            println!("Result part 2: {}", puzzle::day5::part2(&content));
        }
        not_found_day => {
            println!("Day \"{not_found_day}\" implementation was not found");
            println!("To run specific advent of code day, pass the day [1, 2, ..., 25]");
        }
    };
}
