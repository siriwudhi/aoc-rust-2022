use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u32>().expect("Failed to parse calories"))
                .sum()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut h = BinaryHeap::<u32>::new();
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u32>().expect("Failed to parse calories"))
                .sum()
        })
        .for_each(|calories| h.push(calories));

    const NUMBER_OF_ELVES: u32 = 3;
    let mut total_calories = 0u32;
    for _ in 0..NUMBER_OF_ELVES {
        total_calories += h.pop().unwrap_or_default();
    }
    Some(total_calories)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
