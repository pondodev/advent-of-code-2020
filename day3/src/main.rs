use std::fs::File;
use std::io::{self, BufRead};
use std::ops;

fn main() {
    let f = File::open("input").expect("failed to open file");
    let lines = io::BufReader::new(f).lines();

    let input: Vec<Vec<bool>> = lines.map(|line|
        line.expect("failed to read line").chars().map(|c|
            match c {
                '.' => false,
                '#' => true,
                _  => panic!("char not recognised"),
            })
            .collect())
        .collect();

    solve_one(&input);
    solve_two(&input);
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

fn solve_one(input: &Vec<Vec<bool>>) {
    println!("solution one: {}", count_trees(&input, Vec2 { x: 3, y: 1 }));
}

fn solve_two(input: &Vec<Vec<bool>>) {
    let count1 = count_trees(&input, Vec2 { x: 1, y: 1 });
    let count2 = count_trees(&input, Vec2 { x: 3, y: 1 });
    let count3 = count_trees(&input, Vec2 { x: 5, y: 1 });
    let count4 = count_trees(&input, Vec2 { x: 7, y: 1 });
    let count5 = count_trees(&input, Vec2 { x: 1, y: 2 });

    let res = count1 * count2 * count3 * count4 * count5;
    println!("solution two: {}", res);
}

fn count_trees(input: &Vec<Vec<bool>>, dir: Vec2) -> u32 {
    let mut pos = Vec2 { x: 0, y: 0 };

    let bottom = input.len();
    let max_x = input[0].len();

    let mut count = 0;
    while pos.y < bottom {
        if input[pos.y][pos.x] { count += 1 }

        pos += dir;
        pos.x = pos.x % max_x;
    }

    count
}
