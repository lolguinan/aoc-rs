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

fn make_groups(containers: Vec<String>, size: usize) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    let mut counter = 0;
    loop {
        if counter >= containers.len() {
            break;
        }
        if groups.len() == 0 {
            groups.push(Vec::new());
        }
        let group = groups.last().unwrap();
        if group.len() == size {
            groups.push(Vec::new());
        }
        let group_size = groups.len();
        let group = groups.get_mut(group_size - 1).unwrap();
        let container = containers.get(counter).unwrap();
        group.push(container.to_string());
        counter += 1;
    }
    groups
}

fn find_shared_chars(lhs: String, rhs: String) -> Vec<char> {
    let mut shared = Vec::new();
    for a in lhs.chars() {
        for b in rhs.chars() {
            if a == b {
                shared.push(a);
            }
        }
    }
    shared.sort();
    shared.dedup();
    shared
}

fn find_badge(group: Vec<String>) -> char {
    let mut shared = Vec::new();
    for i in 1..group.len() {
        if shared.len() == 0 {
            let lhs = group.get(i - 1).unwrap().to_string();
            let rhs = group.get(i).unwrap().to_string();
            shared = find_shared_chars(lhs, rhs);
        } else {
            let lhs = shared.iter().collect();
            let rhs = group.get(i).unwrap().to_string();
            shared = find_shared_chars(lhs, rhs);
        }
    }
    if shared.len() != 1 {
        panic!("Unexpected badge result: {:#?}", shared);
    }
    *shared.first().unwrap()
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

fn total_priority(groups: Vec<Vec<String>>) -> i32 {
    groups
        .iter()
        .map(|x| find_badge(x.to_vec()))
        .map(|x| get_priority(x))
        .sum()
}

fn main() {
    let containers = parse_input(load_input(3));
    let groups = make_groups(containers, 3);
    let priority = total_priority(groups);
    println!("Answer: {}", priority);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03b_example1() {
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
        let groups = make_groups(containers.to_vec(), 3);
        assert_eq!(2, groups.len());
        let badges: Vec<char> = groups.iter().map(|x| find_badge(x.to_vec())).collect();
        assert_eq!(vec!['r', 'Z'], badges);
        assert_eq!(70, total_priority(groups));
    }
}
