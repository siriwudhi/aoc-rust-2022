use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let middle = line.len() / 2;
                let (first_compartment, second_compartment) = line.split_at(middle);
                let first: HashSet<char> = first_compartment.chars().collect();
                let second: HashSet<char> = second_compartment.chars().collect();
                first
                    .intersection(&second)
                    .map(|c| {
                        if !c.is_alphabetic() {
                            return 0;
                        }
                        if c.is_uppercase() {
                            *c as u32 - 'A' as u32 + 27
                        } else {
                            *c as u32 - 'a' as u32 + 1
                        }
                    })
                    .sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_priorities = 0u32;
    let lines: Vec<_> = input.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        let first: HashSet<char> = lines[i].chars().collect();
        let second: HashSet<char> = lines[i + 1].chars().collect();
        let third: HashSet<char> = lines[i + 2].chars().collect();
        total_priorities += first
            .iter()
            .filter(|c| second.contains(c) && third.contains(c))
            .map(|c| {
                if !c.is_alphabetic() {
                    return 0;
                }
                if c.is_uppercase() {
                    *c as u32 - 'A' as u32 + 27
                } else {
                    *c as u32 - 'a' as u32 + 1
                }
            })
            .sum::<u32>();
    }
    Some(total_priorities)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
