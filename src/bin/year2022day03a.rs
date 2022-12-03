use aoc2022rs::*;

fn parse_input(text: String) -> Vec<String> {
    let mut containers = Vec::new();
    for line in text.trim().lines() {
        if line.len() == 0 {
            continue;
        }
        if line.len() % 2 != 0 {
            panic!("Unbalanced line (len: {}): {:?}", line.len(), line);
        }
        containers.push(line.to_string());
    }
    containers
}

fn find_shared(container: String) -> char {
    let lhs = &container[..(container.len() / 2) + 1];
    let rhs = &container[(container.len() / 2)..];

    for a in lhs.chars() {
        for b in rhs.chars() {
            if a == b {
                return a;
            }
        }
    }
    panic!("No shared items between {:?} and {:?}", lhs, rhs);
}

fn get_priority(item: char) -> i32 {
    if item.is_ascii_lowercase() {
        (item as i32) - (('a' as i32) - 1)
    } else if item.is_ascii_uppercase() {
        (item as i32) - (('A' as i32) - 27)
    } else {
        panic!("Unknown priority for item: {:?}", item);
    }
}

fn total_priority(containers: Vec<String>) -> i32 {
    containers
        .iter()
        .map(|x| find_shared(x.to_string()))
        .map(|x| get_priority(x))
        .sum()
}

fn main() {
    let containers = parse_input(load_input(3));
    let priority = total_priority(containers);
    println!("Answer: {}", priority);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03a_priority() {
        assert_eq!(1, get_priority('a'));
        assert_eq!(26, get_priority('z'));
        assert_eq!(27, get_priority('A'));
        assert_eq!(52, get_priority('Z'));
    }

    #[test]
    fn day03a_example1() {
        let input = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "
        .to_string();
        let containers = parse_input(input);

        let shared: Vec<char> = containers
            .iter()
            .map(|x| find_shared(x.to_string()))
            .collect();
        assert_eq!(vec!['p', 'L', 'P', 'v', 't', 's'], shared);
        let priority = total_priority(containers);
        assert_eq!(157, priority);
    }
}
