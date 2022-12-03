use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let rounds: Vec<(char, char)> = input
        .trim()
        .split("\n")
        .map(|row| (row.chars().nth(0).unwrap(), row.chars().nth(2).unwrap()))
        .collect();

    let mapping = result_mapping_1();

    let score = rounds
        .iter()
        .fold(0, |acc, round| acc + mapping.get(round).unwrap());

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rounds: Vec<(char, char)> = input
        .trim()
        .split("\n")
        .map(|row| (row.chars().nth(0).unwrap(), row.chars().nth(2).unwrap()))
        .collect();

    let mapping = result_mapping_2();

    let score = rounds
        .iter()
        .fold(0, |acc, round| acc + mapping.get(round).unwrap());

    Some(score)
}

fn result_mapping_2() -> HashMap<(char, char), u32> {
    let mut mapping = HashMap::new();
    mapping.insert(('A', 'X'), 3);
    mapping.insert(('A', 'Y'), 4);
    mapping.insert(('A', 'Z'), 8);
    mapping.insert(('B', 'X'), 1);
    mapping.insert(('B', 'Y'), 5);
    mapping.insert(('B', 'Z'), 9);
    mapping.insert(('C', 'X'), 2);
    mapping.insert(('C', 'Y'), 6);
    mapping.insert(('C', 'Z'), 7);
    mapping
}

fn result_mapping_1() -> HashMap<(char, char), u32> {
    let mut mapping = HashMap::new();
    mapping.insert(('A', 'X'), 4);
    mapping.insert(('A', 'Y'), 8);
    mapping.insert(('A', 'Z'), 3);
    mapping.insert(('B', 'X'), 1);
    mapping.insert(('B', 'Y'), 5);
    mapping.insert(('B', 'Z'), 9);
    mapping.insert(('C', 'X'), 7);
    mapping.insert(('C', 'Y'), 2);
    mapping.insert(('C', 'Z'), 6);
    mapping
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
