enum Outcome {
    Win,
    Lose,
    Draw,
}

enum Choices {
    Rock,
    Paper,
    Scissors,
}

impl Choices {
    fn fight(&self, enemy: &Choices) -> Outcome {
        match self {
            Choices::Rock => match enemy {
                Choices::Rock => Outcome::Draw,
                Choices::Paper => Outcome::Lose,
                Choices::Scissors => Outcome::Win,
            },
            Choices::Paper => match enemy {
                Choices::Rock => Outcome::Win,
                Choices::Paper => Outcome::Draw,
                Choices::Scissors => Outcome::Lose,
            },
            Choices::Scissors => match enemy {
                Choices::Rock => Outcome::Lose,
                Choices::Paper => Outcome::Win,
                Choices::Scissors => Outcome::Draw,
            },
        }
    }
    fn reverse_fight(enemy: &Choices, outcome: &Outcome) -> Choices {
        match enemy {
            Choices::Rock => match outcome {
                Outcome::Win => Choices::Paper,
                Outcome::Lose => Choices::Scissors,
                Outcome::Draw => Choices::Rock,
            },
            Choices::Paper => match outcome {
                Outcome::Win => Choices::Scissors,
                Outcome::Lose => Choices::Rock,
                Outcome::Draw => Choices::Paper,
            },
            Choices::Scissors => match outcome {
                Outcome::Win => Choices::Rock,
                Outcome::Lose => Choices::Paper,
                Outcome::Draw => Choices::Scissors,
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_scores = input
        .lines()
        .map(|line| {
            let (elf, you) = line.split_once(' ').expect("Failed to parse input");
            let elf = match elf {
                "A" => Choices::Rock,
                "B" => Choices::Paper,
                "C" => Choices::Scissors,
                _ => panic!("Wrong input format"),
            };
            let you = match you {
                "X" => Choices::Rock,
                "Y" => Choices::Paper,
                "Z" => Choices::Scissors,
                _ => panic!("Wrong input format"),
            };
            let base = match you {
                Choices::Rock => 1,
                Choices::Paper => 2,
                Choices::Scissors => 3,
            };
            let outcome = match you.fight(&elf) {
                Outcome::Win => 6,
                Outcome::Lose => 0,
                Outcome::Draw => 3,
            };
            base + outcome
        })
        .sum();
    Some(total_scores)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_scores = input
        .lines()
        .map(|line| {
            let (elf, outcome_you) = line.split_once(' ').expect("Failed to parse input");
            let elf = match elf {
                "A" => Choices::Rock,
                "B" => Choices::Paper,
                "C" => Choices::Scissors,
                _ => panic!("Wrong input format"),
            };
            let outcome_you = match outcome_you {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => panic!("Wrong input format"),
            };
            let you = Choices::reverse_fight(&elf, &outcome_you);
            let base = match you {
                Choices::Rock => 1,
                Choices::Paper => 2,
                Choices::Scissors => 3,
            };
            let outcome = match outcome_you {
                Outcome::Win => 6,
                Outcome::Lose => 0,
                Outcome::Draw => 3,
            };
            base + outcome
        })
        .sum();
    Some(total_scores)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
