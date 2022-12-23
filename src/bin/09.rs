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

fn simulate_head(head: &Pos, dir: &str) -> Pos {
    match dir {
        "U" => (head.0, head.1 - 1),
        "D" => (head.0, head.1 + 1),
        "L" => (head.0 - 1, head.1),
        "R" => (head.0 + 1, head.1),
        _ => panic!(),
    }
}

fn tail_follows_head(head: &Pos, tail: &Pos) -> Pos {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dx.abs() <= 1 && dy.abs() <= 1 {
        // Adjacent
        *tail
    } else {
        // Step once
        (tail.0 + dx.signum(), tail.1 + dy.signum())
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
            let new_head = simulate_head(&head_pos, dir);
            let new_tail = tail_follows_head(&new_head, &tail_pos);
            head_pos = new_head;
            tail_pos = new_tail;

            tail_visited.insert(tail_pos);
        }
    }

    Some(tail_visited.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let steps = parse_steps(input);

    let mut positions: Vec<Pos> = vec![(0, 0); 10];

    let mut tail_visited: HashSet<Pos> = HashSet::new();
    tail_visited.insert(positions.last().unwrap().clone());

    for (dir, count) in steps {
        for _step in 0..count {
            let head = positions[0];
            let new_head = simulate_head(&head, dir);
            let mut new_positions = vec![new_head];
            for i in 1..positions.len() {
                let previous_segment = new_positions.last().unwrap();
                let tail = positions[i];
                let new_tail = tail_follows_head(&previous_segment, &tail);
                new_positions.push(new_tail);
            }
            positions = new_positions;
            tail_visited.insert(positions.last().unwrap().clone());
        }
    }

    Some(tail_visited.len().try_into().unwrap())
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
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        assert_eq!(part_two(&input), Some(36));
    }
}
