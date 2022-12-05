pub fn part1(_content: &str) -> i32 {
    0
}

pub fn part2(_content: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::puzzle::day5::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let content = read_to_string("./src/puzzle/examples/day5.txt").unwrap();
        let result = part1(&content);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let content = read_to_string("./src/puzzle/examples/day5.txt").unwrap();
        let result = part2(&content);
        assert_eq!(result, 0);
    }
}
