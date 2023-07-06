pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut visible_trees = vec![false; grid.len() * grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            visible_trees[i * grid.len() + j] =
                i == 0 || i == grid.len() - 1 || j == 0 || j == grid.len() - 1
        }
    }

    let mut maximums = grid.first().unwrap().clone();
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            if grid[i][j] > maximums[j] {
                maximums[j] = grid[i][j];
                visible_trees[i * grid.len() + j] = true
            }
        }
    }

    maximums = grid.iter().map(|row| *row.first().unwrap()).collect();
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            if grid[i][j] > maximums[i] {
                maximums[i] = grid[i][j];
                visible_trees[i * grid.len() + j] = true
            }
        }
    }

    maximums = grid.last().unwrap().clone();
    for i in (1..grid.len() - 1).rev() {
        for j in 1..grid.len() - 1 {
            if grid[i][j] > maximums[j] {
                maximums[j] = grid[i][j];
                visible_trees[i * grid.len() + j] = true
            }
        }
    }

    maximums = grid.iter().map(|row| *row.last().unwrap()).collect();
    for i in 1..grid.len() - 1 {
        for j in (1..grid.len() - 1).rev() {
            if grid[i][j] > maximums[i] {
                maximums[i] = grid[i][j];
                visible_trees[i * grid.len() + j] = true
            }
        }
    }

    Some(visible_trees.iter().filter(|t| **t).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut scenic_scores = vec![0u32; grid.len() * grid.len()];
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            let mut scenic_score = 1;

            // top
            let mut total_visibles = 1;
            for item in grid[1..i].iter().rev().map(|e| e[j]) {
                if item >= grid[i][j] {
                    break;
                }
                total_visibles += 1;
            }
            scenic_score *= total_visibles;

            // left
            let mut total_visibles = 1;
            for item in grid[i][1..j].iter().rev() {
                if *item >= grid[i][j] {
                    break;
                }
                total_visibles += 1;
            }
            scenic_score *= total_visibles;

            // right
            let mut total_visibles = 1;
            for item in grid[i][j + 1..grid.len() - 1].iter() {
                if *item >= grid[i][j] {
                    break;
                }
                total_visibles += 1;
            }
            scenic_score *= total_visibles;

            // bottom
            let mut total_visibles = 1;
            for item in grid[i + 1..grid.len() - 1].iter().map(|e| e[j]) {
                if item >= grid[i][j] {
                    break;
                }
                total_visibles += 1;
            }
            scenic_score *= total_visibles;

            scenic_scores[i * grid.len() + j] = scenic_score as u32;
        }
    }

    scenic_scores.into_iter().max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
