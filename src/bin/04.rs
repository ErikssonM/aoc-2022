pub fn part_one(input: &str) -> Option<u32> {
    let range_pairs: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.split(['-', ','])
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut count = 0;

    for pair in range_pairs {
        if pair[0] <= pair[2] && pair[1] >= pair[3] {
            count += 1;
        } else if pair[0] >= pair[2] && pair[1] <= pair[3] {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let range_pairs: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.split(['-', ','])
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut count = 0;

    for pair in range_pairs {
        if pair[0] <= pair[2] && pair[1] >= pair[3] {
            count += 1;
        } else if pair[0] >= pair[2] && pair[1] <= pair[3] {
            count += 1;
        } else if pair[1] >= pair[2] && pair[1] <= pair[3] {
            count += 1;
        } else if pair[0] >= pair[2] && pair[0] <= pair[3] {
            count += 1;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
