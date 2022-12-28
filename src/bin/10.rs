enum Command {
    NOOP,
    ADDX(i32),
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("noop") {
                Command::NOOP
            } else if line.starts_with("addx") {
                let (_, x) = line.split_once(' ').unwrap();
                Command::ADDX(x.parse().unwrap())
            } else {
                panic!("At the disco")
            }
        })
        .collect()
}

fn run(commands: &Vec<Command>) -> i32 {
    let mut x_at_time: Vec<i32> = Vec::new();
    let mut register_x = 1;
    for cmd in commands {
        match cmd {
            Command::NOOP => {
                x_at_time.push(register_x);
            }
            Command::ADDX(x) => {
                x_at_time.push(register_x);
                x_at_time.push(register_x);
                register_x += x;
            }
        }
    }

    let mut signal_strength = 0;
    for t in 0..(x_at_time.len() + 1) {
        if t >= 20 && (t - 20) % 40 == 0 {
            signal_strength += (t as i32) * x_at_time[t - 1];
        }
    }
    signal_strength
}

pub fn part_one(input: &str) -> Option<u32> {
    let commands = parse_commands(input);
    let signal = run(&commands);

    Some(signal.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
