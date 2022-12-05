use std::collections::{HashMap, VecDeque};

use aoc2022rs::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Movement {
    count: u32,
    source: u32,
    target: u32,
}

impl Movement {
    fn new(count: u32, source: u32, target: u32) -> Movement {
        Movement {
            count,
            source,
            target,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    stacks: HashMap<u32, VecDeque<char>>,
    moves: VecDeque<Movement>,
}

impl State {
    fn new() -> State {
        State {
            stacks: HashMap::new(),
            moves: VecDeque::new(),
        }
    }

    fn top_items(&self) -> Vec<char> {
        let mut keys: Vec<u32> = self.stacks.keys().map(|x| x.clone()).collect();
        keys.sort();
        let mut tops = Vec::new();
        for key in keys {
            let stack = self.stacks.get(&key).unwrap();
            tops.push(stack.iter().last().unwrap().clone());
        }
        tops
    }
}

fn parse_input(text: String) -> State {
    let mut state = State::new();
    let mut stack_lines = Vec::new();
    for line in text.lines() {
        if line.trim().len() == 0 {
            continue;
        }
        if line.starts_with("move") {
            let chunks: Vec<&str> = line.split(" ").collect();
            let count: u32 = chunks.get(1).unwrap().parse().unwrap();
            let source: u32 = chunks.get(3).unwrap().parse().unwrap();
            let target: u32 = chunks.get(5).unwrap().parse().unwrap();
            state.moves.push_back(Movement::new(count, source, target));
        } else {
            stack_lines.push(line);
        }
    }

    let mut stack_indices: HashMap<u32, usize> = HashMap::new();
    stack_lines.reverse();
    for line in stack_lines {
        if stack_indices.is_empty() {
            for (index, ch) in line.char_indices() {
                if let Some(stack) = ch.to_digit(10) {
                    stack_indices.entry(stack).or_insert(index);
                }
            }
        } else {
            for (stack, index) in stack_indices.iter() {
                let ch = line.chars().nth(*index).unwrap();
                if ch.is_whitespace() {
                    continue;
                }
                state.stacks.entry(*stack).or_default().push_back(ch);
            }
        }
    }

    state
}

fn apply_moves(initial_state: State) -> State {
    let mut state = initial_state.clone();
    while !state.moves.is_empty() {
        let mov = state.moves.pop_front().unwrap();
        let mut items = Vec::new();
        for _ in 0..mov.count {
            let source = mov.source;
            let stack = state.stacks.get_mut(&source).unwrap();
            let item = stack.pop_back().unwrap();
            items.push(item);
        }
        items.reverse();
        let target = mov.target;
        let stack = state.stacks.get_mut(&target).unwrap();
        for item in items {
            stack.push_back(item);
        }
    }
    state
}

fn main() {
    let initial_state = parse_input(load_input(5));
    let final_state = apply_moves(initial_state);
    let answer: String = final_state.top_items().iter().cloned().collect();
    println!("Answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05b_aexample1_parse() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        "
        .to_string();
        let state = parse_input(input);
        let mut expected_stacks: HashMap<u32, VecDeque<char>> = HashMap::new();
        expected_stacks.entry(1).or_default().push_back('Z');
        expected_stacks.entry(1).or_default().push_back('N');
        expected_stacks.entry(2).or_default().push_back('M');
        expected_stacks.entry(2).or_default().push_back('C');
        expected_stacks.entry(2).or_default().push_back('D');
        expected_stacks.entry(3).or_default().push_back('P');
        assert_eq!(expected_stacks, state.stacks);
        let mut expected_moves: VecDeque<Movement> = VecDeque::new();
        expected_moves.push_back(Movement::new(1, 2, 1));
        expected_moves.push_back(Movement::new(3, 1, 3));
        expected_moves.push_back(Movement::new(2, 2, 1));
        expected_moves.push_back(Movement::new(1, 1, 2));
        assert_eq!(expected_moves, state.moves);
    }

    #[test]
    fn day05b_aexample1() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        "
        .to_string();
        let initial_state = parse_input(input);
        let final_state = apply_moves(initial_state);
        let mut expected_stacks: HashMap<u32, VecDeque<char>> = HashMap::new();
        expected_stacks.entry(1).or_default().push_back('M');
        expected_stacks.entry(2).or_default().push_back('C');
        expected_stacks.entry(3).or_default().push_back('P');
        expected_stacks.entry(3).or_default().push_back('Z');
        expected_stacks.entry(3).or_default().push_back('N');
        expected_stacks.entry(3).or_default().push_back('D');
        assert_eq!(expected_stacks, final_state.stacks);
        assert_eq!(0, final_state.moves.len());
        assert_eq!(vec!['M', 'C', 'D'], final_state.top_items());
    }
}
