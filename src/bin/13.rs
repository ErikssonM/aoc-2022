use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::one_of,
    combinator::{all_consuming, map, map_res, recognize},
    multi::{many0, separated_list0},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    left: Packet,
    right: Packet,
}

fn parse_packet_digit(input: &str) -> IResult<&str, Packet> {
    map_res(
        recognize(terminated(one_of("0123456789"), many0(char('_')))),
        |out: &str| u32::from_str_radix(out, 10).map(|i| Packet::Int(i)),
    )(input)
}

// fn parse_packet_list(input: &str) -> Packet {
//     let (rem, res) =
//         preceded(tag("["), separated_list0(tag(","), parse_packet_digit))(input).unwrap();
//     Packet::List(res)
// }

fn parse_packet_list(input: &str) -> IResult<&str, Packet> {
    map(
        preceded(
            tag("["),
            terminated(
                separated_list0(tag(","), alt((parse_packet_digit, parse_packet_list))),
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

fn parse(input: &str) -> Vec<Pair> {
    all_consuming(separated_list0(tag("\n\n"), parse_pair))(input)
        .unwrap()
        .1
}

// fn parse(input: &str) -> Vec<Pair> {
//     // input.split("\n\n").map(|group| {
//     //     let group = group.split_once('\n').unwrap();
//     // })
// }

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse(input);

    None
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
        assert_eq!(part_one(&input), None);
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
}
