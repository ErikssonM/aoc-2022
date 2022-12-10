use std::collections::HashSet;

use itertools::Itertools;

fn find_distinct_marker(input: &str, n_chars: usize) -> u32 {
    let chars = input.chars().collect_vec();
    let marker_start = chars[..]
        .windows(n_chars)
        .take_while(|ch| HashSet::<char>::from_iter(ch.iter().cloned()).len() != n_chars)
        .count();
    (marker_start + n_chars) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_distinct_marker(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_distinct_marker(input, 14))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
