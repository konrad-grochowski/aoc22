use std::{fs, mem};

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
type Points = i32;

pub fn solve(contents: String) -> () {
    let rounds = contents
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(elf_shape, outcome)| {
            (
                Shape::from_elf_chars(elf_shape.chars().nth(0).unwrap()),
                RoundOutcome::from_xyz(outcome.chars().nth(0).unwrap()),
            )
        })
        .collect::<Vec<(Shape, RoundOutcome)>>();
    let points_sum : Points= rounds.into_iter().map(|(elf_shape, outcome)| outcome.points() + elf_shape.shape_to_achieve_outcome(outcome).points_for_shape()).sum();
    println!("Sum: {}",points_sum);
}


#[derive(Copy, Clone)]
enum RoundOutcome{
    Win,
    Draw,
    Lose
}

impl RoundOutcome {
    pub fn from_xyz(c: char) -> Self {
        use RoundOutcome::*;
        match c {
           'X' =>  Lose, 'Y' => Draw, 'Z' => Win, _ => unreachable!(),
        }
    }
    pub fn points(self) -> Points {
        match self {
            RoundOutcome::Win => 6,
            RoundOutcome::Draw => 3,
            RoundOutcome::Lose => 0,
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
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
    fn points_for_shape(self) -> Points {
        self as Points + 1 // enum values are 0,1,2, point values are 1,2,3
    }
    fn shape_to_achieve_outcome(self, outcome: RoundOutcome) -> Shape {
        match outcome{
            RoundOutcome::Win => Shape::from_i32((self as i32 + 1).rem_euclid(3)).unwrap(),
            RoundOutcome::Draw => self,
            RoundOutcome::Lose =>  Shape::from_i32((self as i32 -1).rem_euclid(3)).unwrap(),
        }
    }
}
