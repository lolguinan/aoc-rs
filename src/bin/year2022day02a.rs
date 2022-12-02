use aoc2022rs::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RpsMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RpsMove::Rock),
            "B" | "Y" => Ok(RpsMove::Paper),
            "C" | "Z" => Ok(RpsMove::Scissors),
            unknown => Err(format!("Not an RPS move: {:?}", unknown)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RpsRound {
    opponent: RpsMove,
    player: RpsMove,
}

impl FromStr for RpsRound {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.splitn(2, " ").collect();
        Ok(RpsRound {
            opponent: parts.get(0).unwrap().parse()?,
            player: parts.get(1).unwrap().parse()?,
        })
    }
}

impl RpsRound {
    fn is_tie(&self) -> bool {
        self.opponent == self.player
    }

    fn is_win(&self) -> bool {
        (self.player == RpsMove::Rock && self.opponent == RpsMove::Scissors)
            || (self.player == RpsMove::Scissors && self.opponent == RpsMove::Paper)
            || (self.player == RpsMove::Paper && self.opponent == RpsMove::Rock)
    }

    fn get_score(&self) -> i32 {
        let mut points = 0;
        points += match self.player {
            RpsMove::Rock => 1,
            RpsMove::Paper => 2,
            RpsMove::Scissors => 3,
        };
        if self.is_win() {
            points += 6;
        } else if self.is_tie() {
            points += 3;
        } else {
            points += 0;
        }
        points
    }
}

fn parse_input(text: String) -> Vec<RpsRound> {
    text.trim().lines().filter_map(|x| x.parse().ok()).collect()
}

fn total_score(rounds: Vec<RpsRound>) -> i32 {
    rounds.iter().map(|x| x.get_score()).sum()
}

fn main() {
    let rounds = parse_input(load_input(2));
    let score = total_score(rounds);
    println!("Answer: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02a_test_parse_bad() {
        let input = "E".to_string();
        let mov = input.parse::<RpsMove>();
        assert!(mov.is_err());
        let rounds = parse_input(input);
        assert_eq!(0, rounds.len());
    }

    #[test]
    fn day02a_test_parse_good() {
        let input = "A Y".to_string();
        let rounds = parse_input(input);
        assert_eq!(1, rounds.len());
        let round = rounds.last().unwrap();
        assert_eq!(
            RpsRound {
                opponent: RpsMove::Rock,
                player: RpsMove::Paper,
            },
            *round
        );
    }

    #[test]
    fn day02a_example1() {
        let input = "
A Y
B X
C Z
        "
        .to_string();
        let rounds = parse_input(input);
        let score = total_score(rounds);
        assert_eq!(15, score);
    }
}
