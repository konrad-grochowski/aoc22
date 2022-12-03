use std::collections::HashSet;

use super::{sum_of_points, Points};

pub fn solve(contents: &str) -> Points{
    let common_items_between_compartments = contents.lines().map(|line| {
        let line_middle = line.len() / 2;
        let (left, right) = line.split_at(line_middle);

        let left = HashSet::<char>::from_iter(left.chars());
        let right = HashSet::<char>::from_iter(right.chars());
        let common_item = left
            .intersection(&right)
            .next()
            .expect("There should be at least one (more spcifically exactly one) common item")
            .clone();
        common_item
    });
    let priorities_sum = sum_of_points(common_items_between_compartments);
    priorities_sum
}
