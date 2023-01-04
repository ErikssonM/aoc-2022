use itertools::Itertools;
use pathfinding::prelude::{astar, bfs};

const HEIGHT: &str = "abcdefghijklmnopqrstuvwxyz";

type Pos = (usize, usize);

type Grid = Vec<Vec<u8>>;

fn parse(input: &str) -> (Grid, Pos, Pos) {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(
                    |(col, c)| match HEIGHT.chars().find_position(|comp_to| c == *comp_to) {
                        Some((height, _)) => (height + 1) as u8,
                        None => {
                            if c == 'S' {
                                start_pos = (row, col);
                                1
                            } else if c == 'E' {
                                end_pos = (row, col);
                                HEIGHT.len() as u8
                            } else {
                                panic!()
                            }
                        }
                    },
                )
                .collect()
        })
        .collect();

    (grid, start_pos, end_pos)
}

fn possible_moves_from(p: &Pos, grid: &Grid) -> Vec<Pos> {
    let mut possible = Vec::new();
    let current_height = grid[p.0][p.1];
    if p.0 != 0 && current_height >= grid[p.0 - 1][p.1] - 1 {
        possible.push((p.0 - 1, p.1));
    }
    if p.0 != grid.len() - 1 && current_height >= grid[p.0 + 1][p.1] - 1 {
        possible.push((p.0 + 1, p.1));
    }
    if p.1 != 0 && current_height >= grid[p.0][p.1 - 1] - 1 {
        possible.push((p.0, p.1 - 1));
    }
    if p.1 != grid[0].len() - 1 && current_height >= grid[p.0][p.1 + 1] - 1 {
        possible.push((p.0, p.1 + 1));
    }
    possible
}

fn possible_moves_from_reversed(p: &Pos, grid: &Grid) -> Vec<Pos> {
    let mut possible = Vec::new();
    let current_height = grid[p.0][p.1];
    if p.0 != 0 && current_height <= grid[p.0 - 1][p.1] + 1 {
        possible.push((p.0 - 1, p.1));
    }
    if p.0 != grid.len() - 1 && current_height <= grid[p.0 + 1][p.1] + 1 {
        possible.push((p.0 + 1, p.1));
    }
    if p.1 != 0 && current_height <= grid[p.0][p.1 - 1] + 1 {
        possible.push((p.0, p.1 - 1));
    }
    if p.1 != grid[0].len() - 1 && current_height <= grid[p.0][p.1 + 1] + 1 {
        possible.push((p.0, p.1 + 1));
    }
    possible
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start, end) = parse(input);

    let result = astar(
        &start,
        |curr| possible_moves_from(curr, &grid).into_iter().map(|p| (p, 1)),
        |&(x, y)| end.0.abs_diff(x) + end.1.abs_diff(y),
        |p| *p == end,
    );

    let steps = result.unwrap().1;

    Some(steps as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, _start, end) = parse(input);

    let result = bfs(
        &end,
        |curr| possible_moves_from_reversed(curr, &grid),
        |p| grid[p.0][p.1] == 1,
    );

    let steps = result.unwrap().len() - 1;

    Some(steps as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
