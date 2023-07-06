pub fn part_one(input: &str) -> Option<String> {
    // build stacks
    let (stacks, commands) = input
        .split_once("\n\n")
        .expect("Failed to split stacks and commands");
    let mut stacks = stacks.lines().rev().skip(1).fold(Vec::new(), |mut acc, e| {
        let total_stacks = (e.chars().count() + 1) / 4;
        if acc.is_empty() {
            acc = vec![vec![]; total_stacks];
        }
        for (i, arr) in acc.iter_mut().enumerate() {
            let c = e.chars().nth(i * 4 + 1).expect("Failed to get crate");
            if c != ' ' {
                arr.push(c)
            }
        }
        acc
    });
    commands.lines().for_each(|cmd| {
        let arr: Vec<_> = cmd.split_whitespace().collect();
        let n = arr[1]
            .parse::<usize>()
            .expect("Failed to parse number of crates");
        let from = arr[3].parse::<usize>().expect("Failed to parse from index") - 1;
        let to = arr[5].parse::<usize>().expect("Failed to parse to index") - 1;
        for _ in 0..n {
            let e = stacks[from].pop().expect("Not enough crates");
            stacks[to].push(e);
        }
    });
    Some(stacks.iter().map(|stack| stack[stack.len() - 1]).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    // build stacks
    let (stacks, commands) = input
        .split_once("\n\n")
        .expect("Failed to split stacks and commands");
    let mut stacks = stacks.lines().rev().skip(1).fold(Vec::new(), |mut acc, e| {
        let total_stacks = (e.chars().count() + 1) / 4;
        if acc.is_empty() {
            acc = vec![vec![]; total_stacks];
        }
        for (i, arr) in acc.iter_mut().enumerate() {
            let c = e.chars().nth(i * 4 + 1).expect("Failed to get crate");
            if c != ' ' {
                arr.push(c)
            }
        }
        acc
    });
    commands.lines().for_each(|cmd| {
        let arr: Vec<_> = cmd.split_whitespace().collect();
        let n = arr[1]
            .parse::<usize>()
            .expect("Failed to parse number of crates");
        let from = arr[3].parse::<usize>().expect("Failed to parse from index") - 1;
        let to = arr[5].parse::<usize>().expect("Failed to parse to index") - 1;
        let from_length = stacks[from].len();
        let mut tail = stacks[from].split_off(from_length - n);
        stacks[to].append(&mut tail);
    });
    Some(stacks.iter().map(|stack| stack[stack.len() - 1]).collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
