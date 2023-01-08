use std::cmp::Ordering;
use std::iter::zip;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{all_consuming, map, map_res, recognize},
    multi::{many0, many1, separated_list0},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    left: Packet,
    right: Packet,
}

fn parse_packet_number(input: &str) -> IResult<&str, Packet> {
    map_res(recognize(many0(one_of("0123456789"))), |out: &str| {
        u32::from_str_radix(out, 10).map(|i| Packet::Int(i))
    })(input)
}

fn parse_packet_list(input: &str) -> IResult<&str, Packet> {
    map(
        preceded(
            tag("["),
            terminated(
                separated_list0(tag(","), alt((parse_packet_number, parse_packet_list))),
                tag("]"),
            ),
        ),
        |list| Packet::List(list),
    )(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    map(
        separated_pair(parse_packet_list, tag("\n"), parse_packet_list),
        |(left, right)| Pair { left, right },
    )(input)
}

fn parse_all_pairs(input: &str) -> Vec<Pair> {
    all_consuming(separated_list0(tag("\n\n"), parse_pair))(input)
        .unwrap()
        .1
}

fn parse_all_lines(input: &str) -> Vec<Packet> {
    all_consuming(separated_list0(many1(tag("\n")), parse_packet_list))(input)
        .unwrap()
        .1
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(s), Packet::Int(o)) => s.cmp(o),
            (Packet::List(s), Packet::List(o)) => {
                let mut zipped = zip(s, o);
                let _ = zipped
                    .take_while_ref(|&(si, oi)| si.cmp(oi) == Ordering::Equal)
                    .collect::<Vec<(&Packet, &Packet)>>();
                if let Some((si, oi)) = zipped.next() {
                    si.cmp(&oi)
                } else {
                    s.len().cmp(&o.len())
                }
            }
            (left_list, Packet::Int(right)) => {
                left_list.cmp(&Packet::List(vec![Packet::Int(*right)]))
            }
            (Packet::Int(left), right_list) => {
                Packet::List(vec![Packet::Int(*left)]).cmp(right_list)
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_all_pairs(input.trim());

    let mut sum = 0;
    for (index, pair) in pairs.iter().enumerate() {
        let Pair { left, right } = pair;

        if left.cmp(&right) == Ordering::Less {
            sum += index + 1;
        }
    }

    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packet_lines = parse_all_lines(input.trim());

    let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    packet_lines.push(divider_1.clone());
    packet_lines.push(divider_2.clone());

    packet_lines.sort_by(Packet::cmp);

    let index_1 = packet_lines
        .iter()
        .find_position(|&p| p.cmp(&divider_1) == Ordering::Equal)
        .unwrap()
        .0
        + 1;

    let index_2 = packet_lines
        .iter()
        .find_position(|&p| p.cmp(&divider_2) == Ordering::Equal)
        .unwrap()
        .0
        + 1;

    Some((index_1 * index_2).try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }

    #[test]
    fn test_parse_list() {
        let input = "[1,1,3,1,1]";
        let expected = Packet::List(
            vec![1, 1, 3, 1, 1]
                .iter()
                .map(|i: &u32| Packet::Int(*i))
                .collect(),
        );

        assert_eq!(parse_packet_list(input).unwrap().1, expected)
    }

    #[test]
    fn test_parse_nested_list() {
        let input = "[[1],4]";
        let expected = Packet::List(vec![Packet::List(vec![Packet::Int(1)]), Packet::Int(4)]);
        assert_eq!(parse_packet_list(input).unwrap().1, expected)
    }

    #[test]
    fn test_parse_basic() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[]
[3]";
        let expected = vec![
            Pair {
                left: Packet::List(
                    vec![1, 1, 3, 1, 1]
                        .iter()
                        .map(|i: &u32| Packet::Int(*i))
                        .collect(),
                ),
                right: Packet::List(
                    vec![1, 1, 5, 1, 1]
                        .iter()
                        .map(|i: &u32| Packet::Int(*i))
                        .collect(),
                ),
            },
            Pair {
                left: Packet::List(vec![]),
                right: Packet::List(vec![Packet::Int(3)]),
            },
        ];
        assert_eq!(parse_all_pairs(input), expected);
    }

    #[test]
    fn test_parse_multi_digit() {
        let input = "[[],[10]]";
        let expected = Packet::List(vec![
            Packet::List(vec![]),
            Packet::List(vec![Packet::Int(10)]),
        ]);
        assert_eq!(parse_packet_list(input).unwrap().1, expected)
    }
}
