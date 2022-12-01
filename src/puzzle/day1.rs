use std::{fs::File, io::Read};

pub fn run(mut file: File) -> i32 {
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut grouped_numbers: Vec<i32> = vec![];
    let mut acc = vec![];

    // iterate over everline creating specific groups
    for value in content.split('\n') {
        if value.is_empty() {
            grouped_numbers.push(acc.iter().sum::<i32>());
            acc = vec![];
            continue;
        }

        acc.push(value.parse::<i32>().unwrap());
    }

    // return the highest value of all grouped numbers
    grouped_numbers.sort_unstable();
    *grouped_numbers.last().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::puzzle::day1::run;
    use std::fs::File;

    #[test]
    fn example() {
        let file = File::open("./src/puzzle/day1-example.txt").unwrap();

        let result = run(file);
        assert_eq!(result, 24000);
    }
}
