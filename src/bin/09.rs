use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn x_diff(self: &Point, other: &Point) -> i32 {
        self.x - other.x
    }

    fn y_diff(self: &Point, other: &Point) -> i32 {
        self.y - other.y
    }

    fn add(self: &mut Point, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut knots = vec![Point::new(0, 0); 2];
    let mut tail_pos = HashSet::new();
    for cmd in input.lines() {
        let (direction, n) = cmd
            .split_once(' ')
            .expect("Failed to split command into direction and n");
        let n = n.parse::<usize>().expect("Failed to parse n into integer");

        let vector = match direction {
            "R" => Point::new(1, 0),
            "L" => Point::new(-1, 0),
            "U" => Point::new(0, 1),
            "D" => Point::new(0, -1),
            _ => panic!("Failed to parse direction"),
        };

        for _i in 0..n {
            knots[0].add(&vector);
            for point_idx in 1..knots.len() {
                let curr_head_pos = knots[point_idx - 1];
                let mut curr_tail_pos = knots[point_idx];

                let x_diff = curr_head_pos.x_diff(&curr_tail_pos);
                let x_abs_diff = x_diff.abs();
                let y_diff = curr_head_pos.y_diff(&curr_tail_pos);
                let y_abs_diff = y_diff.abs();

                let total_diff = x_abs_diff + y_abs_diff;
                let mut tail_vector = Point::new(0, 0);
                if total_diff < 2 {
                    continue;
                } else if total_diff == 2 && x_abs_diff > y_abs_diff {
                    if x_diff < 0 {
                        tail_vector.x = -1
                    } else {
                        tail_vector.x = 1
                    }
                } else if total_diff == 2 && x_abs_diff < y_abs_diff {
                    if y_diff < 0 {
                        tail_vector.y = -1
                    } else {
                        tail_vector.y = 1
                    }
                } else if total_diff >= 3 {
                    tail_vector = Point::new(1, 1);
                    if x_diff < 0 {
                        tail_vector.x = -1
                    }
                    if y_diff < 0 {
                        tail_vector.y = -1
                    }
                }

                curr_tail_pos.add(&tail_vector);
                knots[point_idx - 1] = curr_head_pos;
                knots[point_idx] = curr_tail_pos;
            }
            tail_pos.insert(knots[knots.len() - 1]);
        }
    }
    Some(tail_pos.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut knots = vec![Point::new(0, 0); 10];
    let mut tail_pos = HashSet::new();
    for cmd in input.lines() {
        let (direction, n) = cmd
            .split_once(' ')
            .expect("Failed to split command into direction and n");
        let n = n.parse::<usize>().expect("Failed to parse n into integer");

        let vector = match direction {
            "R" => Point::new(1, 0),
            "L" => Point::new(-1, 0),
            "U" => Point::new(0, 1),
            "D" => Point::new(0, -1),
            _ => panic!("Failed to parse direction"),
        };

        for _i in 0..n {
            knots[0].add(&vector);
            for point_idx in 1..knots.len() {
                let curr_head_pos = knots[point_idx - 1];
                let mut curr_tail_pos = knots[point_idx];

                let x_diff = curr_head_pos.x_diff(&curr_tail_pos);
                let x_abs_diff = x_diff.abs();
                let y_diff = curr_head_pos.y_diff(&curr_tail_pos);
                let y_abs_diff = y_diff.abs();

                let total_diff = x_abs_diff + y_abs_diff;
                let mut tail_vector = Point::new(0, 0);
                if total_diff < 2 {
                    continue;
                } else if total_diff == 2 && x_abs_diff > y_abs_diff {
                    if x_diff < 0 {
                        tail_vector.x = -1
                    } else {
                        tail_vector.x = 1
                    }
                } else if total_diff == 2 && x_abs_diff < y_abs_diff {
                    if y_diff < 0 {
                        tail_vector.y = -1
                    } else {
                        tail_vector.y = 1
                    }
                } else if total_diff >= 3 {
                    tail_vector = Point::new(1, 1);
                    if x_diff < 0 {
                        tail_vector.x = -1
                    }
                    if y_diff < 0 {
                        tail_vector.y = -1
                    }
                }

                curr_tail_pos.add(&tail_vector);

                knots[point_idx - 1] = curr_head_pos;
                knots[point_idx] = curr_tail_pos;
            }
            tail_pos.insert(knots[knots.len() - 1]);
        }
    }
    Some(tail_pos.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
