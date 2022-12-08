use std::collections::HashSet;

use itertools::Itertools;

static PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn fill_hash_set(items: &str) -> HashSet<char> {
    let mut hs = HashSet::new();
    items.chars().for_each(|item| {
        hs.insert(item);
    });
    hs
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks: Vec<(HashSet<char>, HashSet<char>)> = input
        .lines()
        .map(|row| {
            let (first, second) = row.split_at(row.len() / 2);
            (fill_hash_set(first), fill_hash_set(second))
        })
        .collect();

    let items: Vec<char> = rucksacks
        .iter()
        .flat_map(|(first, second)| first.intersection(second).cloned().collect_vec())
        .collect();

    Some(
        items
            .iter()
            .map(|c: &char| PRIORITIES.find(*c).unwrap() as u32 + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let groups: Vec<(HashSet<char>, HashSet<char>, HashSet<char>)> = input
        .lines()
        .tuples()
        .map(|(first, second, third)| {
            (
                fill_hash_set(first),
                fill_hash_set(second),
                fill_hash_set(third),
            )
        })
        .collect();

    let badges: Vec<char> = groups
        .iter()
        .flat_map(|(first, second, third)| {
            let first_and_second: HashSet<char> = first.intersection(second).cloned().collect();
            first_and_second.intersection(third).cloned().collect_vec()
        })
        .collect();

    Some(
        badges
            .iter()
            .map(|c| PRIORITIES.find(*c).unwrap() as u32 + 1)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
