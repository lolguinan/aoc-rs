use aoc2022rs::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

impl RpsMove {
    fn fromstr(s: String) -> RpsMove {
        match s.as_str() {
            "A" => RpsMove::Rock,
            "B" => RpsMove::Paper,
            "C" => RpsMove::Scissors,
            unknown => panic!("Unknown RpsMove: {:?}", unknown),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RpsOutcome {
    Lose,
    Draw,
    Win,
}

impl RpsOutcome {
    fn fromstr(s: String) -> RpsOutcome {
        match s.as_str() {
            "X" => RpsOutcome::Lose,
            "Y" => RpsOutcome::Draw,
            "Z" => RpsOutcome::Win,
            unknown => panic!("Not an RPS strategy: {:?}", unknown),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RpsRoundStrategy {
    opponent: RpsMove,
    outcome: RpsOutcome,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RpsRound {
    opponent: RpsMove,
    player: RpsMove,
}

impl RpsRound {
    fn from_strategy(strat: RpsRoundStrategy) -> RpsRound {
        let moves = vec![RpsMove::Rock, RpsMove::Paper, RpsMove::Scissors];
        let move_count: i32 = moves.len().try_into().unwrap();
        let opponent_idx: i32 = moves
            .iter()
            .position(|x| *x == strat.opponent)
            .unwrap()
            .try_into()
            .unwrap();
        let player_idx = match strat.outcome {
            RpsOutcome::Lose => opponent_idx - 1,
            RpsOutcome::Draw => opponent_idx,
            RpsOutcome::Win => opponent_idx + 1,
        };
        let player_idx: usize = ((player_idx + move_count) % move_count).try_into().unwrap();
        let player = moves.get(player_idx).unwrap();
        RpsRound {
            opponent: strat.opponent,
            player: *player,
        }
    }

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

fn parse_input(text: String) -> Vec<RpsRoundStrategy> {
    let mut rounds = Vec::new();
    for line in text.trim().lines() {
        if line.len() == 0 {
            continue;
        }
        let parts: Vec<_> = line.splitn(2, " ").collect();
        if parts.len() != 2 {
            panic!("Unknown input line: {:?}", line);
        }
        let opponent = RpsMove::fromstr(parts.get(0).unwrap().to_string());
        let outcome = RpsOutcome::fromstr(parts.get(1).unwrap().to_string());
        rounds.push(RpsRoundStrategy { opponent, outcome });
    }
    rounds
}

fn total_score(rounds: Vec<RpsRoundStrategy>) -> i32 {
    rounds
        .iter()
        .map(|x| RpsRound::from_strategy(*x))
        .map(|x| x.get_score())
        .sum()
}

fn main() {
    let strats = parse_input(load_input(2));
    let score = total_score(strats);
    println!("Answer: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02b_example1() {
        let input = "
A Y
B X
C Z
        "
        .to_string();
        let rounds = parse_input(input);
        let score = total_score(rounds);
        assert_eq!(12, score);
    }
}
