use std::collections::HashSet;

type Pos = (isize, isize);

fn parse_steps(input: &str) -> Vec<(&str, u32)> {
    input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(dir, steps)| (dir, steps.parse::<u32>().unwrap()))
                .unwrap()
        })
        .collect()
}

fn simulate_step(head: &Pos, tail: &Pos, dir: &str) -> (Pos, Pos) {
    match dir {
        "U" => {
            let new_head = (head.0, head.1 + 1);
            let mut new_tail = tail.clone();
            let dx = new_head.0 - tail.0;
            let dy = new_head.1 - tail.1;
            if dy.abs() > 1 {
                new_tail.1 += 1;
                if dx == 1 {
                    new_tail.0 += 1;
                } else if dx == -1 {
                    new_tail.0 -= 1;
                }
            }

            (new_head, new_tail)
        }
        "D" => {
            let new_head = (head.0, head.1 - 1);
            let mut new_tail = tail.clone();
            let dx = new_head.0 - tail.0;
            let dy = new_head.1 - tail.1;
            if dy.abs() > 1 {
                new_tail.1 -= 1;
                if dx == 1 {
                    new_tail.0 += 1;
                } else if dx == -1 {
                    new_tail.0 -= 1;
                }
            }
            (new_head, new_tail)
        }
        "L" => {
            let new_head = (head.0 - 1, head.1);
            let mut new_tail = tail.clone();
            let dx = new_head.0 - tail.0;
            let dy = new_head.1 - tail.1;
            if dx.abs() > 1 {
                new_tail.0 -= 1;
                if dy == 1 {
                    new_tail.1 += 1;
                } else if dy == -1 {
                    new_tail.1 -= 1;
                }
            }
            (new_head, new_tail)
        }
        "R" => {
            let new_head = (head.0 + 1, head.1);
            let mut new_tail = tail.clone();
            let dx = new_head.0 - tail.0;
            let dy = new_head.1 - tail.1;
            if dx.abs() > 1 {
                new_tail.0 += 1;
                if dy == 1 {
                    new_tail.1 += 1;
                } else if dy == -1 {
                    new_tail.1 -= 1;
                }
            }
            (new_head, new_tail)
        }
        _ => panic!("Panic"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let steps = parse_steps(input);

    let mut head_pos: Pos = (0, 0);
    let mut tail_pos: Pos = (0, 0);

    let mut tail_visited: HashSet<Pos> = HashSet::new();
    tail_visited.insert(tail_pos);

    for (dir, count) in steps {
        for _step in 0..count {
            let (head, tail) = simulate_step(&head_pos, &tail_pos, dir);
            head_pos = head;
            tail_pos = tail;

            tail_visited.insert(tail_pos);
        }
    }

    Some(tail_visited.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let steps = parse_steps(input);

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
