use std::{
    collections::HashSet,
    convert::identity,
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
        .map(|line| line.split_at(line.len() / 2))
        .map(|tuple| {
            let first_pocket: HashSet<char> = HashSet::from_iter(tuple.0.chars());
            let secound_pocket: HashSet<char> = HashSet::from_iter(tuple.1.chars());
            let intersection = first_pocket
                .intersection(&secound_pocket)
                .collect::<Vec<&char>>();

            match intersection.first() {
                Some(intersection) => Some(intersection.clone().clone()),
                None => None,
            }
        })
        .filter_map(identity)
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

pub fn part2(mut file: &File) -> i32 {
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut content = String::new();
    let mut file = <&std::fs::File>::clone(&file);
    file.read_to_string(&mut content).unwrap();

    content
        .split('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
        // we get an empty chunk -> filter it out
        .take_while(|chunk| chunk.len() > 1)
        .map(|chunk| {
            let first_bag: HashSet<char> = HashSet::from_iter(chunk[0].chars());
            let secound_bag: HashSet<char> = HashSet::from_iter(chunk[1].chars());
            let third_bag: HashSet<char> = HashSet::from_iter(chunk[2].chars());

            // use two intersections (i did not find a better way to do it, without many conversions)
            first_bag
                .intersection(&secound_bag)
                .map(char::to_owned)
                .collect::<HashSet<char>>()
                .intersection(&third_bag)
                .map(char::to_owned)
                .collect::<Vec<char>>()
                .first()
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
    use std::fs::File;

    #[test]
    fn test_part1() {
        let file = File::open("./src/puzzle/day3-example.txt").unwrap();
        let result = part1(&file);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part2() {
        let file = File::open("./src/puzzle/day3-example.txt").unwrap();
        let result = part2(&file);
        assert_eq!(result, 70);
    }
}
