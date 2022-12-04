use aoc2022rs::*;

#[derive(Debug, Clone, Copy)]
struct IdRange {
    start: i32,
    stop: i32,
}

fn parse_input(text: String) -> Vec<(IdRange, IdRange)> {
    let mut pairs = Vec::new();
    for line in text.trim().lines() {
        if line.len() == 0 {
            continue;
        }
        let mut lhs = None;
        let mut rhs = None;
        if let Some((pair0, pair1)) = line.split_once(",") {
            if let Some((start, stop)) = pair0.split_once("-") {
                lhs = Some(IdRange {
                    start: start.parse().unwrap(),
                    stop: stop.parse().unwrap(),
                });
            }
            if let Some((start, stop)) = pair1.split_once("-") {
                rhs = Some(IdRange {
                    start: start.parse().unwrap(),
                    stop: stop.parse().unwrap(),
                });
            }
        }
        if lhs.is_none() || rhs.is_none() {
            panic!("Unparse line: {:?}", line);
        }
        pairs.push((lhs.unwrap(), rhs.unwrap()));
    }
    pairs
}

fn has_overlap(lhs: IdRange, rhs: IdRange) -> bool {
    (lhs.start >= rhs.start && lhs.start <= rhs.stop)
        || (lhs.stop >= rhs.start && lhs.stop <= rhs.stop)
        || (rhs.start >= lhs.start && rhs.start <= lhs.stop)
        || (rhs.stop >= lhs.start && rhs.stop <= lhs.stop)
}

fn total_overlap_pairs(pairs: Vec<(IdRange, IdRange)>) -> usize {
    pairs.iter().filter(|x| has_overlap(x.0, x.1)).count()
}

fn main() {
    let pairs = parse_input(load_input(4));
    let overlaps = total_overlap_pairs(pairs);
    println!("Answer: {}", overlaps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04a_example1() {
        let input = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "
        .to_string();
        let pairs = parse_input(input);
        assert_eq!(6, pairs.len());
        let overlaps = total_overlap_pairs(pairs);
        assert_eq!(4, overlaps);
    }
}
