fn parse_grid(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    let grid_side = grid.len();

    let mut visible: Vec<Vec<bool>> = (0..grid_side).map(|_r| vec![false; grid_side]).collect();

    let mut max_height_from_top = vec![-1; grid_side];
    let mut max_height_from_left = vec![-1; grid_side];
    for r in 0..grid_side {
        for c in 0..grid_side {
            let height = grid[r][c];

            if height > max_height_from_top[c] {
                visible[r][c] = true;
                max_height_from_top[c] = height;
            }

            if height > max_height_from_left[r] {
                visible[r][c] = true;
                max_height_from_left[r] = height;
            }
        }
    }

    let mut max_height_from_bottom = vec![-1; grid_side];
    let mut max_height_from_right = vec![-1; grid_side];
    for r in (0..grid_side).rev() {
        for c in (0..grid_side).rev() {
            let height = grid[r][c];

            if height > max_height_from_bottom[c] {
                visible[r][c] = true;
                max_height_from_bottom[c] = height;
            }

            if height > max_height_from_right[r] {
                visible[r][c] = true;
                max_height_from_right[r] = height;
            }
        }
    }

    let count = visible
        .iter()
        .map(|row| row.iter().fold(0, |acc, v| if *v { acc + 1 } else { acc }))
        .sum();

    Some(count)
}

fn visibility_score(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let grid_size = grid.len();
    let height = grid[row][col];

    let mut score = 0;
    let mut current_row = row;
    let up = loop {
        if current_row == 0 {
            break score;
        }
        current_row -= 1;
        if height <= grid[current_row][col] {
            break score + 1;
        } else {
            score += 1;
        }
    };

    let mut score = 0;
    let mut current_row = row;
    let down = loop {
        if current_row == grid_size - 1 {
            break score;
        }
        current_row += 1;
        if height <= grid[current_row][col] {
            break score + 1;
        } else {
            score += 1;
        }
    };

    let mut score = 0;
    let mut current_col = col;
    let left = loop {
        if current_col == 0 {
            break score;
        }
        current_col -= 1;
        if height <= grid[row][current_col] {
            break score + 1;
        } else {
            score += 1;
        }
    };

    let mut score = 0;
    let mut current_col = col;
    let right = loop {
        if current_col == grid_size - 1 {
            break score;
        }
        current_col += 1;
        if height <= grid[row][current_col] {
            break score + 1;
        } else {
            score += 1;
        }
    };

    up * down * left * right
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let grid_size = grid.len();

    let mut max_score = 0;

    for r in 0..grid_size {
        for c in 0..grid_size {
            let score = visibility_score(&grid, r, c);
            max_score = if score > max_score { score } else { max_score };
        }
    }

    Some(max_score as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
