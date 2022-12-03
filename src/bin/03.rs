use itertools::Itertools;

static PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks: Vec<(&str, &str)> = input
        .split("\n")
        .map(|row| row.split_at(row.len() / 2))
        .collect();

    let items: Vec<char> = rucksacks
        .iter()
        .map(|(first, second)| {
            first
                .chars()
                .into_iter()
                .find_map(|fc| second.chars().find(|sc| *sc == fc))
                .unwrap()
        })
        .collect();

    Some(
        items
            .iter()
            .map(|c| PRIORITIES.find(*c).unwrap() as u32 + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let groups: Vec<(&str, &str, &str)> = input.split("\n").tuples().collect();

    let badges: Vec<char> = groups
        .into_iter()
        .map(|(first, second, third)| {
            first
                .chars()
                .into_iter()
                .find_map(|fc| {
                    if second.chars().contains(&fc) && third.chars().contains(&fc) {
                        Some(fc)
                    } else {
                        None
                    }
                })
                .unwrap()
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
        assert_eq!(part_two(&input), None);
    }
}
