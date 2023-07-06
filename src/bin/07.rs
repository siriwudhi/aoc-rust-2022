use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut pwd = "/".to_string();
    input.split_terminator('$').skip(1).for_each(|ch| {
        let (cmd, output) = ch.split_once('\n').expect("Failed to split cmd and output");
        let cmd = cmd.trim();
        if cmd.starts_with("cd") {
            let path = cmd
                .split_once(' ')
                .expect("Failed to extract path from 'cd'")
                .1
                .trim();
            if path == "/" {
                pwd = "/".to_string();
            } else if path == ".." {
                let mut new_path = pwd
                    .rsplit_once('/')
                    .expect("Failed to extract parent path")
                    .0;
                if new_path.is_empty() {
                    new_path = "/"
                }
                pwd = new_path.to_string();
            } else if !path.is_empty() {
                if pwd == "/" {
                    pwd = format!("/{}", path);
                } else {
                    pwd = format!("{}/{}", pwd, path);
                }
            }
        } else if cmd == "ls" {
            let total_size = output
                .lines()
                .map(|line| {
                    if line.starts_with("dir") {
                        return 0;
                    }
                    line.split_once(' ')
                        .expect("Failed to extract file size")
                        .0
                        .parse::<u32>()
                        .expect("Failed to parse file size")
                })
                .sum::<u32>();
            if pwd == "/" {
                dirs.entry(pwd.clone())
                    .and_modify(|e| *e += total_size)
                    .or_insert(total_size);
            } else {
                let arr = pwd.split('/').skip(1).collect::<Vec<_>>();
                for i in 0..arr.len() {
                    let pwd = format!("/{}", arr[..=i].join("/"));
                    dirs.entry(pwd)
                        .and_modify(|e| *e += total_size)
                        .or_insert(total_size);
                }
            }
        }
    });
    Some(dirs.values().filter(|v| **v <= 100_000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut pwd = "/".to_string();
    input.split_terminator('$').skip(1).for_each(|ch| {
        let (cmd, output) = ch.split_once('\n').expect("Failed to split cmd and output");
        let cmd = cmd.trim();
        if cmd.starts_with("cd") {
            let path = cmd
                .split_once(' ')
                .expect("Failed to extract path from 'cd'")
                .1
                .trim();
            if path == "/" {
                pwd = "/".to_string();
            } else if path == ".." {
                let mut new_path = pwd
                    .rsplit_once('/')
                    .expect("Failed to extract parent path")
                    .0;
                if new_path.is_empty() {
                    new_path = "/"
                }
                pwd = new_path.to_string();
            } else if !path.is_empty() {
                if pwd == "/" {
                    pwd = format!("/{}", path);
                } else {
                    pwd = format!("{}/{}", pwd, path);
                }
            }
        } else if cmd == "ls" {
            let total_size = output
                .lines()
                .map(|line| {
                    if line.starts_with("dir") {
                        return 0;
                    }
                    line.split_once(' ')
                        .expect("Failed to extract file size")
                        .0
                        .parse::<u32>()
                        .expect("Failed to parse file size")
                })
                .sum::<u32>();
            dirs.entry("/".to_string())
                .and_modify(|e| *e += total_size)
                .or_insert(total_size);
            if pwd != "/" {
                let arr = pwd.split('/').skip(1).collect::<Vec<_>>();
                for i in 0..arr.len() {
                    let pwd = format!("/{}", arr[..=i].join("/"));
                    dirs.entry(pwd)
                        .and_modify(|e| *e += total_size)
                        .or_insert(total_size);
                }
            }
        }
    });
    let unused_space = 70_000_000 - dirs["/"];
    if unused_space > 30_000_000 {
        return Some(0);
    }
    let needed_space = 30_000_000 - unused_space;
    dirs.values().filter(|v| **v > needed_space).copied().min()
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
