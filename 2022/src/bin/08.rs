use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

pub fn part_one(input: &str) -> Option<u32> {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|number| (number as u32 - b'0' as u32))
                .collect_vec()
        })
        .collect_vec();

    let mut sum = 0;
    for row in 1..trees.len() - 1 {
        for col in 1..trees[0].len() - 1 {
            let tree = trees[row][col];
            let (left, right) = trees[row].split_at(col);
            let tree_row = trees.iter().map(|tree_line| tree_line[col]).collect_vec();
            let (up, down) = tree_row.split_at(row);

            if left.iter().all(|v| *v < tree)
                || right.iter().skip(1).all(|v| *v < tree)
                || up.iter().all(|v| *v < tree)
                || down.iter().skip(1).all(|v| *v < tree)
            {
                sum += 1;
            }
        }
    }
    sum += (trees.len() - 2) * 2 + trees[0].len() * 2;
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|number| (number as u32 - b'0' as u32))
                .collect_vec()
        })
        .collect_vec();

    let mut views = Vec::with_capacity(trees.len() * trees[0].len());
    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            let tree = trees[row][col];
            let (left, right) = trees[row].split_at(col);
            let tree_row = trees.iter().map(|tree_line| tree_line[col]).collect_vec();
            let (up, down) = tree_row.split_at(row);

            let scenic_score = left
                .iter()
                .rev()
                .fold_while(0, |accum, x| {
                    if *x < tree {
                        Continue(accum + 1)
                    } else {
                        Done(accum + 1)
                    }
                })
                .into_inner()
                * right
                    .iter()
                    .skip(1)
                    .fold_while(0, |accum, x| {
                        if *x < tree {
                            Continue(accum + 1)
                        } else {
                            Done(accum + 1)
                        }
                    })
                    .into_inner()
                * up.iter()
                    .rev()
                    .fold_while(0, |accum, x| {
                        if *x < tree {
                            Continue(accum + 1)
                        } else {
                            Done(accum + 1)
                        }
                    })
                    .into_inner()
                * down
                    .iter()
                    .skip(1)
                    .fold_while(0, |accum, x| {
                        if *x < tree {
                            Continue(accum + 1)
                        } else {
                            Done(accum + 1)
                        }
                    })
                    .into_inner();

            views.push(scenic_score as u32);
        }
    }
    views.into_iter().max()
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
