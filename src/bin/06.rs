use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let (i, _) = input
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .find(|(_, chunk)| {
            let mut hs = HashSet::new();
            chunk.iter().all(|c| hs.insert(c))
        })
        .expect("Failed to find a start-of-package");
    Some(i as u32 + 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (i, _) = input
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
        .find(|(_, chunk)| {
            let mut hs = HashSet::new();
            chunk.iter().all(|c| hs.insert(c))
        })
        .expect("Failed to find a start-of-package");
    Some(i as u32 + 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}
