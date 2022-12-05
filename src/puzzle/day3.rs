use std::collections::HashSet;

pub fn part1(content: &str) -> i32 {
    content
        .split('\n')
        .map(|line| line.split_at(line.len() / 2))
        .filter_map(|tuple| {
            let first_pocket = tuple.0.chars().collect::<HashSet<char>>();
            let secound_pocket = tuple.1.chars().collect::<HashSet<char>>();
            let intersection = first_pocket
                .intersection(&secound_pocket)
                .collect::<Vec<&char>>();

            intersection.first().map(|i| *<&char>::clone(i))
        })
        .map(|found_item| {
            // A starts at 65, but it should be 27, so substract difference
            if found_item.is_uppercase() {
                return found_item as i32 - 38;
            }

            // a starts at 97, but it should be 1, so substract difference
            found_item as i32 - 96
        })
        .sum::<i32>()
}

pub fn part2(content: &str) -> i32 {
    content
        .split('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
        // we get an empty chunk at the end -> filter it out
        .take_while(|chunk| chunk.len() > 1)
        .map(|chunk| {
            chunk
                .iter()
                .map(|c| c.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>()
                .into_iter()
                .reduce(|acc, ele| {
                    acc.intersection(&ele)
                        .map(char::to_owned)
                        .collect::<HashSet<char>>()
                })
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .to_owned()
        })
        .map(|found_item| {
            // A starts at 65, but it should be 27, so substract difference
            if found_item.is_uppercase() {
                return found_item as i32 - 38;
            }

            // a starts at 97, but it should be 1, so substract difference
            found_item as i32 - 96
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use crate::puzzle::day3::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let content = read_to_string("./src/puzzle/examples/day3.txt").unwrap();
        let result = part1(&content);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part2() {
        let content = read_to_string("./src/puzzle/examples/day3.txt").unwrap();
        let result = part2(&content);
        assert_eq!(result, 70);
    }
}
