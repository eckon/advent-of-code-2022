pub fn part1(content: &str) -> i32 {
    let mut grouped_numbers = group_content(content);

    // return the highest value of all grouped numbers
    grouped_numbers.sort_unstable();
    grouped_numbers.reverse();
    grouped_numbers.iter().take(1).sum::<i32>()
}

pub fn part2(content: &str) -> i32 {
    let mut grouped_numbers = group_content(content);

    // return a sum of the three highes values
    grouped_numbers.sort_unstable();
    grouped_numbers.reverse();
    grouped_numbers.iter().take(3).sum::<i32>()
}

fn group_content(content: &str) -> Vec<i32> {
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
    use crate::puzzle::day1::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let content = read_to_string("./src/puzzle/examples/day1.txt").unwrap();
        let result = part1(&content);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part2() {
        let content = read_to_string("./src/puzzle/examples/day1.txt").unwrap();
        let result = part2(&content);
        assert_eq!(result, 45000);
    }
}
