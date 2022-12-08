use std::cmp::min;

fn parse_stacks(stacks_input: &str) -> Vec<Vec<char>> {
    let all_lines: Vec<&str> = stacks_input.lines().collect();
    let legend = all_lines[all_lines.len() - 1];
    let stack_lines = all_lines[0..all_lines.len() - 1].to_vec();

    // println!("All: {all_lines:?}");
    // println!("Legend: {legend:?}");
    // println!("stack_lines: {stack_lines:?}");

    let count = legend.chars().filter(|c| c.is_alphanumeric()).count();

    let mut stacks: Vec<Vec<char>> = (0..count).map(|_i| Vec::new()).collect();

    for stack_line in stack_lines.into_iter().rev() {
        // println!("Stack line: {stack_line}");
        for i in 0..count {
            let letter = stack_line.chars().nth((i * 4) + 1).unwrap();
            // println!("Letter: {letter}");

            if letter.is_alphanumeric() {
                stacks[i].push(letter);
            }
        }
    }

    stacks
}

fn parse_procedure(procedure_input: &str) -> Vec<(u32, u32, u32)> {
    procedure_input
        .lines()
        .map(|line| {
            let items: Vec<&str> = line.split(' ').collect();
            (
                items[1].parse().unwrap(),
                items[3].parse().unwrap(),
                items[5].parse().unwrap(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks_input, procedure_input) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks_input);
    let procedure = parse_procedure(procedure_input);

    for (count, from_stack, to_stack) in procedure.into_iter() {
        let mut cur = count;

        'procedure: while 0 < cur {
            match stacks[(from_stack - 1) as usize].pop() {
                Some(crejt) => {
                    stacks[(to_stack - 1) as usize].push(crejt);
                }
                None => {
                    break 'procedure;
                }
            }

            cur -= 1;
        }
    }

    let tops: String = stacks
        .iter()
        .map(|stack| stack.last().cloned().unwrap())
        .collect();

    Some(tops)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks_input, procedure_input) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks_input);
    let procedure = parse_procedure(procedure_input);

    for (count, from_stack, to_stack) in procedure.into_iter() {
        // println!("{stacks:?}");
        let stack_size = stacks[(from_stack - 1) as usize].len();
        let mid_index = stack_size - min(stack_size, count as usize);

        let keep = stacks[(from_stack - 1) as usize][0..mid_index].to_owned();
        let mov = stacks[(from_stack - 1) as usize][mid_index..].to_owned();

        stacks[(from_stack - 1) as usize] = keep;
        stacks[(to_stack - 1) as usize].extend_from_slice(&mov);
    }

    let tops: String = stacks
        .iter()
        .map(|stack| stack.last().cloned().unwrap())
        .collect();

    Some(tops)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
