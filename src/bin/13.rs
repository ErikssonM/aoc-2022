use std::cmp::Ordering;
use std::iter::zip;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{all_consuming, map, map_res, recognize},
    multi::{many0, separated_list0},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
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
    // println!("Parsing digit: {input}");
    map_res(recognize(many0(one_of("0123456789"))), |out: &str| {
        u32::from_str_radix(out, 10).map(|i| Packet::Int(i))
    })(input)
}

fn parse_packet_list(input: &str) -> IResult<&str, Packet> {
    // println!("\nParsing packet_list: {input}");
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
    // println!("\nParsing pair: {input}");
    map(
        separated_pair(parse_packet_list, tag("\n"), parse_packet_list),
        |(left, right)| Pair { left, right },
    )(input)
}

fn parse(input: &str) -> Vec<Pair> {
    all_consuming(separated_list0(tag("\n\n"), parse_pair))(input)
        .unwrap()
        .1

    // let result = input.split("\n\n").map(|s| parse_pair(s)).collect_vec();

    // for res in result {
    //     if matches!(res, Result::Err(_)) {
    //         println!("{res:?}");
    //     }
    // }

    // vec![]
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Packet::Int(s), Packet::Int(o)) => s.cmp(o),
            (Packet::List(s), Packet::List(o)) => {
                // println!("Comparing {s:?} and {o:?}");
                let mut zipped = zip(s, o);
                let _ = zipped
                    .take_while_ref(|&(si, oi)| *si == *oi)
                    .collect::<Vec<(&Packet, &Packet)>>();
                if let Some((si, oi)) = zipped.next() {
                    // println!("Itemwise comparison, {si:?}, {oi:?}");
                    si.partial_cmp(&oi).unwrap()
                } else {
                    s.len().cmp(&o.len())
                }
            }
            (left_list, Packet::Int(right)) => left_list
                .partial_cmp(&Packet::List(vec![Packet::Int(*right)]))
                .unwrap(),
            (Packet::Int(left), right_list) => Packet::List(vec![Packet::Int(*left)])
                .partial_cmp(right_list)
                .unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse(input.trim());

    let mut sum = 0;
    for (index, pair) in pairs.iter().enumerate() {
        let Pair { left, right } = pair;
        println!("\nComparing {pair:?}");

        if left < right {
            println!("They are in correct order");
            sum += index + 1;
        }
    }

    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
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
        assert_eq!(parse(input), expected);
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
