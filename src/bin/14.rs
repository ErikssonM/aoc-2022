use itertools::Itertools;
use ndarray::Array2;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, map_res, recognize},
    multi::{many0, separated_list1},
    sequence::separated_pair,
    IResult,
};

type Point = (u32, u32);

type Rock = Vec<Point>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Sand,
    Rock,
}

type Cave = Array2<Tile>;

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(recognize(many0(one_of("0123456789"))), |out: &str| {
        u32::from_str_radix(out, 10)
    })(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    separated_pair(parse_number, tag(","), parse_number)(input)
}

fn parse_rocks(input: &str) -> IResult<&str, Rock> {
    separated_list1(tag(" -> "), parse_point)(input)
}

fn parse_all_rocks(input: &str) -> Vec<Rock> {
    all_consuming(separated_list1(line_ending, parse_rocks))(input)
        .unwrap()
        .1
}

fn build_cave(rocks: &Vec<Rock>, with_floor: bool) -> Cave {
    let (max_right, max_down) = rocks
        .iter()
        .flatten()
        .fold((0, 0), |(old_x, old_y), (x, y)| {
            (old_x.max(*x), old_y.max(*y))
        });

    let mut cave = Array2::from_elem(
        ((max_right + 200) as usize, (max_down + 2) as usize),
        Tile::Empty,
    );

    // + 200 is a very beautiful hack. Much efficient.

    if with_floor {
        cave.append(
            ndarray::Axis(1),
            Array2::from_elem(((max_right + 200) as usize, 1), Tile::Rock).view(),
        )
        .unwrap();
    }

    let all_rock_points: Vec<Point> = rocks
        .iter()
        .flat_map(|rock| {
            rock.iter()
                .tuple_windows()
                .flat_map(|((x1, y1), (x2, y2))| {
                    if x1 == x2 {
                        (u32::min(*y1, *y2)..u32::max(*y1, *y2) + 1)
                            .map(|y| (*x1, y))
                            .collect_vec()
                    } else {
                        (u32::min(*x1, *x2)..u32::max(*x1, *x2) + 1)
                            .map(|x| (x, *y1))
                            .collect_vec()
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    for (x, y) in all_rock_points {
        cave[(x as usize, y as usize)] = Tile::Rock;
    }

    cave
}

fn simulate(cave: &mut Cave) -> u32 {
    let mut sand_counter = 0;
    let (_, max_y) = cave.dim();

    'sand_falling: loop {
        let mut sand_position = (500, 0);

        loop {
            let sp = sand_position;
            // Check OOB
            if (sp.1 + 1) == max_y {
                break 'sand_falling sand_counter;
            }

            // Check down
            if cave[(sp.0, sp.1 + 1)] == Tile::Empty {
                // Move down
                sand_position = (sp.0, sp.1 + 1);
            } else if cave[(sp.0 - 1, sp.1 + 1)] == Tile::Empty {
                // Move down and left
                sand_position = (sp.0 - 1, sp.1 + 1);
            } else if cave[(sp.0 + 1, sp.1 + 1)] == Tile::Empty {
                // Move down and right
                sand_position = (sp.0 + 1, sp.1 + 1);
            } else {
                // Stay here
                cave[sp] = Tile::Sand;
                sand_counter += 1;
                if sand_position == (500, 0) {
                    break 'sand_falling sand_counter;
                } else {
                    break;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let all_rocks: Vec<Rock> = parse_all_rocks(input.trim());

    let mut cave = build_cave(&all_rocks, false);

    let sand = simulate(&mut cave);

    Some(sand)
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_rocks: Vec<Rock> = parse_all_rocks(input.trim());

    let mut cave = build_cave(&all_rocks, true);

    let sand = simulate(&mut cave);

    Some(sand)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
