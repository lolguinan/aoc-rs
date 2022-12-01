use aoc2022rs::*;

fn parse_input(text: String) -> Vec<Vec<i32>> {
    let mut groups = Vec::new();
    for line in text.trim().lines() {
        if groups.len() == 0 {
            groups.push(Vec::new());
        }
        if line.trim().len() == 0 {
            groups.push(Vec::new());
            continue;
        }
        let value = line.parse().unwrap();
        let group = groups.last_mut().unwrap();
        group.push(value);
    }
    groups
}

fn most_calories(groups: Vec<Vec<i32>>, top: usize) -> i32 {
    let mut sums: Vec<i32> = groups.iter().map(|x| x.iter().sum()).collect();
    sums.sort();
    sums.iter().rev().take(top).sum::<i32>()
}

fn main() {
    let groups = parse_input(load_input(1));
    let largest = most_calories(groups, 3);
    println!("Answer: {}", largest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01b_example1() {
        let input = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        "
        .to_string();
        let groups = parse_input(input);
        let largest = most_calories(groups, 3);
        assert_eq!(45000, largest);
    }
}
