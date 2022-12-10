use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Fs {
    File { size: usize },
    Directory { contents: HashMap<String, Fs> },
}

impl Fs {
    fn get_size(&self) -> usize {
        match self {
            Fs::File { size } => *size,
            Fs::Directory { contents } => contents.values().map(|value| value.get_size()).sum(),
        }
    }

    fn merge_at_path(&self, path: &[String], other: &Fs) -> Fs {
        match path {
            [] => self.merge(other),
            [next, rest @ ..] => match self {
                Fs::File { .. } => panic!("Cannot merge to file"),
                Fs::Directory { contents } => {
                    let mut new_contents = contents.clone();
                    new_contents.insert(
                        next.clone(),
                        contents.get(next).unwrap().merge_at_path(rest, other),
                    );
                    Fs::Directory {
                        contents: new_contents,
                    }
                }
            },
        }
    }

    fn merge(&self, other: &Fs) -> Fs {
        match self {
            Fs::File { .. } => self.clone(),
            Fs::Directory { contents } => {
                let mut merged = contents.clone();

                match other {
                    Fs::File { .. } => todo!(),
                    Fs::Directory { contents } => {
                        contents.iter().for_each(|(fname, fs)| {
                            if let Some(matching) = contents.get(fname) {
                                merged.insert(fname.clone(), matching.merge(fs));
                            } else {
                                merged.insert(fname.clone(), fs.clone());
                            }
                        });
                        Fs::Directory { contents: merged }
                    }
                }
            }
        }
    }

    fn collect_directory_sizes(&self, collector: &mut Vec<usize>) {
        match self {
            Fs::File { .. } => (),
            Fs::Directory { contents } => {
                collector.push(self.get_size());
                contents.values().for_each(|fs| {
                    fs.collect_directory_sizes(collector);
                });
            }
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd { path: String },
    Ls { contents: HashMap<String, Fs> },
}

fn parse_commands(input: &str) -> Vec<Command> {
    let mut lines = input.lines().peekable();
    let mut commands = Vec::new();

    'outer: loop {
        if let Some(line) = lines.next() {
            // println!("Matching line: {line}");
            match line.split(' ').collect::<Vec<&str>>()[..] {
                ["$", "cd", dir] => {
                    commands.push(Command::Cd {
                        path: dir.to_owned(),
                    });
                }
                ["$", "ls"] => {
                    let mut ls_info = HashMap::new();
                    'inner: loop {
                        if let Some(next) = lines.peek() {
                            if !next.starts_with('$') {
                                let (p1, p2) =
                                    lines.next().and_then(|ls| ls.split_once(' ')).unwrap();
                                if p1 == "dir" {
                                    ls_info.insert(
                                        p2.to_owned(),
                                        Fs::Directory {
                                            contents: HashMap::new(),
                                        },
                                    );
                                } else {
                                    let size = p1.parse::<usize>().unwrap();
                                    ls_info.insert(p2.to_owned(), Fs::File { size });
                                }
                            } else {
                                break 'inner;
                            }
                        } else {
                            commands.push(Command::Ls { contents: ls_info });
                            break 'outer; // iterator finished
                        }
                    }
                    commands.push(Command::Ls { contents: ls_info });
                }
                _ => panic!("Unexpected things happened"),
            }
        } else {
            break;
        }
    }
    commands
}

fn parse_into_fs(input: &str) -> Fs {
    let commands = parse_commands(input);

    // println!("{commands:?}");

    let mut current_path = Vec::new();
    let mut root = Fs::Directory {
        contents: HashMap::new(),
    };

    for command in commands {
        // println!("\nExecuting command: {command:?}");
        // println!("Current path: {current_path:?}");
        // println!("Current state: {root:?}");

        match command {
            Command::Cd { path } => {
                if path == "/" {
                    current_path = Vec::new();
                } else if path == ".." {
                    current_path.pop();
                } else {
                    current_path.push(path);
                }
            }
            Command::Ls { contents } => {
                // println!("LS contents: {contents:?}");
                root = root.merge_at_path(&current_path[..], &Fs::Directory { contents });
            }
        }
    }

    root
}

pub fn part_one(input: &str) -> Option<u32> {
    let root = parse_into_fs(input);

    let mut collector = Vec::new();

    root.collect_directory_sizes(&mut collector);

    Some(
        collector
            .iter()
            .filter(|size| **size <= 100000)
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let root = parse_into_fs(input);

    let total_space = 70000000;
    let required_space = 30000000;
    let total_used_space = root.get_size();

    let min_required_for_delete = required_space - (total_space - total_used_space);

    let mut collector = Vec::new();
    root.collect_directory_sizes(&mut collector);

    Some(
        *(collector
            .iter()
            .filter(|size| **size >= min_required_for_delete)
            .min()
            .unwrap()) as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
