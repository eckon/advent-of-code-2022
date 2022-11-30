mod puzzle;

fn main() {
    // get first passed argument to cli
    let Ok(day) = std::env::args()
        .nth(1)
        .unwrap_or_else(|| 0.to_string())
        .parse::<u8>() else {
            println!("Passed number can be parsed into int");
            return
        };

    match day {
        1 => puzzle::day1::run(),
        not_found_day => {
            println!("Day \"{not_found_day}\" implementation was not found");
            println!("To run specific advent of code day, pass the day [1, 2, ..., 25]");
        }
    };
}
