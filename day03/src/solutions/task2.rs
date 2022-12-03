use std::collections::HashSet;

use itertools::Itertools;

use super::{Points, sum_of_points};

const ELVES_IN_GROUP: usize = 3;
pub fn solve(contents: &str)-> Points {
    let groups = contents.lines().chunks(ELVES_IN_GROUP);
    let group_items = groups.into_iter().map(|group| {
        group
            .map(|rucksack_content| HashSet::<char>::from_iter(rucksack_content.chars()))
            .reduce(|accum, item| {
                accum
                    .intersection(&item)
                    .copied()
                    .collect::<HashSet<char>>()
            })
            .expect("Group should not be empty")
            .into_iter()
            .next()
            .expect("There should be at least one (more specifically exactly one) common element")
    });
    let priorities_sum = sum_of_points(group_items);
    priorities_sum
}
