pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let section_ids: Vec<_> = line
                    .split(&['-', ','])
                    .map(|c| c.parse::<u32>().expect("Failed to parse input"))
                    .collect();
                if section_ids.len() < 4 {
                    panic!("Input in wrong format");
                }
                let first_pair_contains =
                    section_ids[0] <= section_ids[2] && section_ids[1] >= section_ids[3];
                let second_pair_contains =
                    section_ids[2] <= section_ids[0] && section_ids[3] >= section_ids[1];
                first_pair_contains || second_pair_contains
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let section_ids: Vec<_> = line
                    .split(&['-', ','])
                    .map(|c| c.parse::<u32>().expect("Failed to parse input"))
                    .collect();
                if section_ids.len() < 4 {
                    panic!("Input in wrong format");
                }
                let first_pair_overlaps =
                    section_ids[0] <= section_ids[2] && section_ids[1] >= section_ids[2];
                let second_pair_overlaps =
                    section_ids[2] <= section_ids[0] && section_ids[3] >= section_ids[0];
                first_pair_overlaps || second_pair_overlaps
            })
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
