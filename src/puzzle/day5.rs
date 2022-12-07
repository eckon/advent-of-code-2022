pub fn part1(content: &str) -> String {
    let start_positions = parse_input(content);
    let mut stacks = create_stacks(&start_positions);

    let mut found_empty_line = false;
    for line in content.split('\n') {
        if line.is_empty() {
            found_empty_line = true;
            continue;
        }

        if !found_empty_line && !line.is_empty() {
            continue;
        }

        // only handle lines, that are after the first init and after the empty line
        let mut iter = line.split_whitespace();
        let amount = iter.nth(1).unwrap().parse::<usize>().unwrap();
        let from = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 1..=amount {
            let popped = &stacks[from].pop().unwrap();
            let _ = &stacks[to].push(popped.to_string());
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect::<String>()
}

fn create_stacks(parsed_input: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![];
    for (index, row) in parsed_input.iter().enumerate() {
        // init vectors
        if index == 0 {
            for _ in row {
                result.push(vec![]);
            }
            continue;
        }

        for (i, chunk) in row.iter().enumerate() {
            if chunk.is_empty() {
                continue;
            }

            result[i].push(chunk.to_string());
        }
    }

    result
}

fn parse_input(content: &str) -> Vec<Vec<String>> {
    let mut parsed_rows = content
        .split('\n')
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(<[char]>::to_vec)
                // filter out spacing and combine it into a vec of strings
                .map(|chunk| {
                    chunk
                        .into_iter()
                        .filter(|r| *r != ' ')
                        .filter(|r| *r != '[')
                        .filter(|r| *r != ']')
                        .map(|c| c.to_string())
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    // reverse as the stack needs to be build from the bottom up
    parsed_rows.reverse();
    parsed_rows
}

pub fn part2(content: &str) -> String {
    let start_positions = parse_input(content);
    let mut stacks = create_stacks(&start_positions);

    let mut found_empty_line = false;
    for line in content.split('\n') {
        if line.is_empty() {
            found_empty_line = true;
            continue;
        }

        if !found_empty_line && !line.is_empty() {
            continue;
        }

        // only handle lines, that are after the first init and after the empty line
        let mut iter = line.split_whitespace();
        let amount = iter.nth(1).unwrap().parse::<usize>().unwrap();
        let from = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = iter.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        // ugly implementation, as I can not get drain/append to be working, so we create slices
        // and reverse them afterwards to push them in the correct order
        let mut popped = vec![];
        for _ in 1..=amount {
            popped.push(stacks[from].pop().unwrap().clone());
        }
        popped.reverse();

        for pop in popped {
            let _ = &stacks[to].push(pop.clone().to_string());
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::puzzle::day5::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parsing() {
        let result = parse_input("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3\n\n");
        assert_eq!(result[0], ["1", "2", "3"]);
        assert_eq!(result[1], ["Z", "M", "P"]);
        assert_eq!(result[2], ["N", "C", ""]);
        assert_eq!(result[3], ["", "D", ""]);
    }

    #[test]
    fn test_stacks() {
        let result = create_stacks(&vec![
            vec!["1".to_string()],
            vec!["Z".to_string()],
            vec!["N".to_string()],
        ]);
        assert_eq!(result, vec![vec!["Z".to_string(), "N".to_string()]]);
    }

    #[test]
    fn test_part1() {
        let content = read_to_string("./src/puzzle/examples/day5.txt").unwrap();
        let result = part1(&content);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let content = read_to_string("./src/puzzle/examples/day5.txt").unwrap();
        let result = part2(&content);
        assert_eq!(result, "MCD");
    }
}
