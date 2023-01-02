use std::collections::VecDeque;

#[derive(Debug)]
enum Value {
    Old,
    Const(u64),
}

#[derive(Debug)]
enum Operation {
    Add(Value, Value),
    Mult(Value, Value),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisible_by: u64,
    true_monkey_index: usize,
    false_monkey_index: usize,
}

impl Value {
    fn get_value(&self, old_level: &u64) -> u64 {
        match self {
            Value::Old => old_level.clone(),
            Value::Const(v) => v.clone(),
        }
    }
}

impl Operation {
    fn perform(&self, worry_level: &u64) -> u64 {
        match self {
            Operation::Add(v1, v2) => v1.get_value(worry_level) + v2.get_value(worry_level),
            Operation::Mult(v1, v2) => v1.get_value(worry_level) * v2.get_value(worry_level),
        }
    }
}

fn parse_value(input: &str) -> Value {
    if input == "old" {
        Value::Old
    } else {
        Value::Const(input.parse().unwrap())
    }
}

fn parse_operation(input: &str) -> Operation {
    let tokens: Vec<&str> = input.trim().split(' ').collect();
    let value_1 = parse_value(tokens[0]);
    let value_2 = parse_value(tokens[2]);
    match tokens[1] {
        "+" => Operation::Add(value_1, value_2),
        "*" => Operation::Mult(value_1, value_2),
        _ => panic!("Could not parse"),
    }
}

fn into_monkey(input: &str) -> Monkey {
    let mut lines = input.trim().lines();

    // Throw away first line containing monkey index
    lines.next();

    // Parse starting items
    let starting_items_line = lines.next().unwrap().trim();
    assert!(starting_items_line.starts_with("Starting items:"));
    let items: VecDeque<u64> = starting_items_line
        .trim_start_matches("Starting items: ")
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect();

    // Parse Operation
    let operation_line = lines.next().unwrap().trim();
    assert!(operation_line.starts_with("Operation:"));
    let operation = parse_operation(operation_line.trim_start_matches("Operation: new = "));

    // Parse test
    let test_line = lines.next().unwrap().trim();
    assert!(test_line.starts_with("Test:"));
    let test_divisible_by = test_line
        .trim_start_matches("Test: divisible by ")
        .parse::<u64>()
        .unwrap();

    // Parse true case
    let true_case_line = lines.next().unwrap().trim();
    assert!(true_case_line.starts_with("If true:"));
    let true_monkey_index = true_case_line
        .trim_start_matches("If true: throw to monkey ")
        .parse::<usize>()
        .unwrap();

    // Parse false case
    let false_case_line = lines.next().unwrap().trim();
    assert!(false_case_line.starts_with("If false:"));
    let false_monkey_index = false_case_line
        .trim_start_matches("If false: throw to monkey ")
        .parse::<usize>()
        .unwrap();

    Monkey {
        items,
        operation,
        test_divisible_by,
        true_monkey_index,
        false_monkey_index,
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(into_monkey).collect()
}

fn perform_monkey_business(
    mut monkeys: Vec<Monkey>,
    rounds: usize,
    decrease_worry_level: bool,
) -> u64 {
    let product_all_monkey_tests = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.test_divisible_by);

    let mut monkey_inspected_item: Vec<u64> = vec![0; monkeys.len()];
    for _ in 0..rounds {
        let mut monkey_index = 0;

        while monkey_index < monkeys.len() {
            while let Some(item) = monkeys[monkey_index].items.pop_front() {
                let worry_level_after_operation = monkeys[monkey_index].operation.perform(&item);
                let final_worry_level = if decrease_worry_level {
                    (worry_level_after_operation / 3) % product_all_monkey_tests
                } else {
                    worry_level_after_operation % product_all_monkey_tests
                };
                let test_divisible =
                    final_worry_level.clone() % monkeys[monkey_index].test_divisible_by == 0;
                let send_item_to_monkey = if test_divisible {
                    monkeys[monkey_index].true_monkey_index
                } else {
                    monkeys[monkey_index].false_monkey_index
                };
                monkeys[send_item_to_monkey]
                    .items
                    .push_back(final_worry_level);

                monkey_inspected_item[monkey_index] += 1;
            }

            monkey_index += 1;
        }
    }

    monkey_inspected_item.sort();

    let two_most_monkeying = monkey_inspected_item[monkey_inspected_item.len() - 2].clone()
        * monkey_inspected_item[monkey_inspected_item.len() - 1].clone();

    two_most_monkeying
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = parse_monkeys(input);

    let monkey_business = perform_monkey_business(monkeys, 20, true);

    Some(monkey_business)
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkeys = parse_monkeys(input);

    let monkey_business = perform_monkey_business(monkeys, 10000, false);

    Some(monkey_business)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
