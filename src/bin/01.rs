pub fn part_one(input: &str) -> Option<u32> {
    let elf_food = parse(&input);

    let mut max = 0;
    for food_list in elf_food.iter() {
        let elf_sum = food_list.iter().sum();
        if elf_sum > max {
            max = elf_sum
        };
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_sums: Vec<u32> = parse(&input).iter().map(|elf| elf.iter().sum()).collect();
    elf_sums.sort();
    elf_sums.reverse();
    Some(elf_sums[0..3].iter().sum())
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .trim_end()
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|cal| cal.parse::<u32>().expect("Parse Error!"))
                .collect()
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
