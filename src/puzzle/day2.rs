pub fn part1(content: &str) -> i32 {
    content.split('\n').map(calculate_points).sum::<i32>()
}

pub fn part2(content: &str) -> i32 {
    content.split('\n').map(calculate_faked_points).sum::<i32>()
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Rock,
    Paper,
    Sissor,
}

impl Token {
    fn beats(&self, match_up: &Self) -> i8 {
        match self {
            token if token == match_up => 0,
            token if token.clone().get_losing_match() == *match_up => -1,
            _ => 1,
        }
    }

    const fn get_winning_match(&self) -> Self {
        match self {
            Self::Rock => Self::Sissor,
            Self::Paper => Self::Rock,
            Self::Sissor => Self::Paper,
        }
    }

    const fn get_losing_match(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Sissor,
            Self::Sissor => Self::Rock,
        }
    }

    const fn get_value(&self) -> i8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Sissor => 3,
        }
    }

    fn get_match_points(&self, match_up: &Self) -> i8 {
        match self.beats(match_up) {
            0 => 3, // draw
            1 => 6, // win
            _ => 0, // lose
        }
    }
}

fn calculate_points(game: &str) -> i32 {
    if game.is_empty() {
        return 0;
    }

    let pairs = game.split(' ').collect::<Vec<&str>>();
    let my_sign = pairs.get(1).unwrap().to_owned();
    let enemy_sign = pairs.first().unwrap().to_owned();

    let my_token = match my_sign {
        "X" => Token::Rock,
        "Y" => Token::Paper,
        "Z" => Token::Sissor,
        _ => return 0,
    };

    let enemy_token = match enemy_sign {
        "A" => Token::Rock,
        "B" => Token::Paper,
        "C" => Token::Sissor,
        _ => return 0,
    };

    let token_points = my_token.get_value();
    let match_points = my_token.get_match_points(&enemy_token);

    (token_points + match_points).into()
}

fn calculate_faked_points(game: &str) -> i32 {
    if game.is_empty() {
        return 0;
    }

    let pairs = game.split(' ').collect::<Vec<&str>>();
    let game_decision = pairs.get(1).unwrap().to_owned();
    let enemy_sign = pairs.first().unwrap().to_owned();

    let enemy_token = match enemy_sign {
        "A" => Token::Rock,
        "B" => Token::Paper,
        "C" => Token::Sissor,
        _ => return 0,
    };

    let my_token = match game_decision {
        "X" => enemy_token.get_winning_match(),
        "Y" => enemy_token.clone(),
        "Z" => enemy_token.get_losing_match(),
        _ => return 0,
    };

    let token_points = my_token.get_value();
    let match_points = my_token.get_match_points(&enemy_token);

    (token_points + match_points).into()
}

#[cfg(test)]
mod tests {
    use crate::puzzle::day2::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let content = read_to_string("./src/puzzle/examples/day2.txt").unwrap();
        let result = part1(&content);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part1_rock() {
        assert_eq!(calculate_points("A X"), 4);
        assert_eq!(calculate_points("B X"), 1);
        assert_eq!(calculate_points("C X"), 7);
    }

    #[test]
    fn test_part1_paper() {
        assert_eq!(calculate_points("A Y"), 8);
        assert_eq!(calculate_points("B Y"), 5);
        assert_eq!(calculate_points("C Y"), 2);
    }

    #[test]
    fn test_part1_sissor() {
        assert_eq!(calculate_points("A Z"), 3);
        assert_eq!(calculate_points("B Z"), 9);
        assert_eq!(calculate_points("C Z"), 6);
    }

    #[test]
    fn test_part2() {
        let content = read_to_string("./src/puzzle/examples/day2.txt").unwrap();
        let result = part2(&content);
        assert_eq!(result, 12);
    }
}
