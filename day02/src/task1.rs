pub fn solve(contents: String) -> () {
    let rounds = contents
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(elf_shape, my_shape)| {
            (
                Shape::from_elf_chars(elf_shape.chars().nth(0).unwrap()),
                Shape::from_my_chars(my_shape.chars().nth(0).unwrap()),
            )
        })
        .collect::<Vec<(Shape, Shape)>>();
    let points_sum : Points= rounds.into_iter().map(|(elf_shape, my_shape)| my_shape.points_for_shape() + my_shape.round_outcome(elf_shape)).sum();
    println!("Sum: {}",points_sum);
}


use std::{fs, mem};



type Points = i64;
#[derive(Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn from_elf_chars(c: char) -> Self {
        use Shape::*;
        match c {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => unreachable!(),
        }
    }
    fn from_my_chars(c: char) -> Self {
        use Shape::*;
        match c {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => unreachable!(),
        }
    }
    fn points_for_shape(self) -> Points {
        self as Points
    }
    fn round_outcome(self, opponent_shape: Shape) -> Points {
        const VARIANT_COUNT: Points = 3;
        let shapes_difference = (self as Points - opponent_shape as Points).rem_euclid(VARIANT_COUNT);
        match shapes_difference {
            // shapes were the same => draw
            0 => 3,
            // self played rock to scissors, paper to rock, or scissors to paper => win
            1 => 6,
            // opponent played rock to scissors, paper to rock, or scissors to paper => lose
            2 => 0,
            _ => unreachable!(),
        }
    }
}
