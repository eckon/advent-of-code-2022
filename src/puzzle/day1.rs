use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

pub fn part1(file: &File) -> i32 {
    let mut grouped_numbers = group_file(file);

    // return the highest value of all grouped numbers
    grouped_numbers.sort_unstable();
    grouped_numbers.reverse();
    grouped_numbers.iter().take(1).sum::<i32>()
}

pub fn part2(file: &File) -> i32 {
    let mut grouped_numbers = group_file(file);

    // return a sum of the three highes values
    grouped_numbers.sort_unstable();
    grouped_numbers.reverse();
    grouped_numbers.iter().take(3).sum::<i32>()
}

fn group_file(mut file: &File) -> Vec<i32> {
    // as we re-read the same file, set the cursor to the beginning
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut content = String::new();
    let mut file = <&std::fs::File>::clone(&file);
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

    grouped_numbers
}

#[cfg(test)]
mod tests {
    use crate::puzzle::day1::{part1, part2};
    use std::fs::File;

    #[test]
    fn test_part1() {
        let file = File::open("./src/puzzle/day1-example.txt").unwrap();
        let result = part1(&file);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part2() {
        let file = File::open("./src/puzzle/day1-example.txt").unwrap();
        let result = part2(&file);
        assert_eq!(result, 45000);
    }
}
