enum ApplicationState {
    Normal,
    Paused(i32),
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut total_signal_strength = 0;
    let mut x = 1;
    let mut cycle = 0;
    let mut lines = input.lines();
    let mut state = ApplicationState::Normal;
    loop {
        cycle += 1;
        if (cycle - 20) % 40 == 0 {
            total_signal_strength += cycle * x;
            if cycle == 220 {
                break;
            }
        }
        match state {
            ApplicationState::Paused(v) => {
                x += v;
                state = ApplicationState::Normal;
            }
            ApplicationState::Normal => {
                if let Some(cmd) = lines.next() {
                    if cmd.starts_with("addx") {
                        let v = cmd.split_once(' ').unwrap().1.parse::<i32>().unwrap();
                        state = ApplicationState::Paused(v);
                    }
                }
            }
        }
    }
    Some(total_signal_strength)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut x = 1;
    let mut cycle = 0;
    let mut lines = input.lines();
    let mut state = ApplicationState::Normal;
    loop {
        if cycle % 40 == 0 {
            println!();
        }
        if (cycle % 40) >= x - 1 && (cycle % 40) <= x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        cycle += 1;
        match state {
            ApplicationState::Paused(v) => {
                x += v;
                state = ApplicationState::Normal;
            }
            ApplicationState::Normal => {
                if let Some(cmd) = lines.next() {
                    if cmd.starts_with("addx") {
                        let v = cmd.split_once(' ').unwrap().1.parse::<i32>().unwrap();
                        state = ApplicationState::Paused(v);
                    }
                } else {
                    break;
                }
            }
        }
    }
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
