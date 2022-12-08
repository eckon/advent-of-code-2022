use std::collections::HashSet;

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
pub fn part1(content: &str) -> usize {
    let chars = content.chars().collect::<Vec<char>>();
    for (i, _) in chars.iter().enumerate() {
        let first_index = i as i32 - 3;
        if first_index < 0 {
            continue;
        }

        let first_index = first_index as usize;
        let current_chars = &chars[first_index..=i];

        if current_chars[0] != current_chars[1]
            && current_chars[0] != current_chars[2]
            && current_chars[0] != current_chars[3]
            && current_chars[1] != current_chars[2]
            && current_chars[1] != current_chars[3]
            && current_chars[2] != current_chars[3]
        {
            return i + 1;
        }
    }
    0
}

pub fn part2(content: &str) -> usize {
    let size = 14;
    content
        .chars()
        .collect::<Vec<char>>()
        .windows(size)
        .position(|chunk| {
            let mut char_set = HashSet::new();
            for c in chunk.iter() {
                let _ = char_set.insert(c);
            }

            if char_set.len() == size {
                return true;
            }

            false
        })
        .unwrap()
        + size
}

#[cfg(test)]
mod tests {
    use crate::puzzle::day6::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let content = read_to_string("./src/puzzle/examples/day6.txt").unwrap();
        let result = part1(&content);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part2() {
        let content = read_to_string("./src/puzzle/examples/day6.txt").unwrap();
        let result = part2(&content);
        assert_eq!(result, 19);
    }
}
