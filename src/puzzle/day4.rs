use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

pub fn part1(mut file: &File) -> i32 {
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut content = String::new();
    let mut file = <&std::fs::File>::clone(&file);
    file.read_to_string(&mut content).unwrap();

    content
        .split('\n')
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap_or_default(),
                split.next().unwrap_or_default(),
            )
        })
        .filter(|pair| !pair.0.is_empty())
        .filter(|pair| {
            let first_values = pair
                .0
                .split('-')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i32>>();

            let secound_values = pair
                .1
                .split('-')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i32>>();

            if first_values[0] <= secound_values[0] && first_values[1] >= secound_values[1] {
                return true;
            }

            if secound_values[0] <= first_values[0] && secound_values[1] >= first_values[1] {
                return true;
            }

            false
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(mut file: &File) -> i32 {
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut content = String::new();
    let mut file = <&std::fs::File>::clone(&file);
    file.read_to_string(&mut content).unwrap();

    content
        .split('\n')
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap_or_default(),
                split.next().unwrap_or_default(),
            )
        })
        .filter(|pair| !pair.0.is_empty())
        .filter(|pair| {
            let first_values = pair
                .0
                .split('-')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i32>>();

            let secound_values = pair
                .1
                .split('-')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i32>>();

            let first_range = first_values[0]..=first_values[1];
            if first_range.contains(&secound_values[0]) || first_range.contains(&secound_values[1])
            {
                return true;
            }

            let secound_range = secound_values[0]..=secound_values[1];
            if secound_range.contains(&first_values[0]) || secound_range.contains(&first_values[1])
            {
                return true;
            }

            false
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::puzzle::day4::*;
    use std::fs::File;

    #[test]
    fn test_part1() {
        let file = File::open("./src/puzzle/day4-example.txt").unwrap();
        let result = part1(&file);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let file = File::open("./src/puzzle/day4-example.txt").unwrap();
        let result = part2(&file);
        assert_eq!(result, 4);
    }
}
