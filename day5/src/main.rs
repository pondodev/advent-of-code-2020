use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input").expect("failed to open file");
    let lines = io::BufReader::new(f).lines();

    let input: Vec<String> = lines.map(|line|
        line.expect("failed to read line"))
        .collect();

    solve_one(&input);
    solve_two(&input);
}

#[derive(Debug)]
struct MinMax {
    min: i32,
    max: i32,
}

impl MinMax {
    fn take_upper(&mut self) {
        let mid = (self.min + self.max) / 2;
        self.min = mid;
    }

    fn take_lower(&mut self) {
        let mid = (self.min + self.max) / 2;
        self.max = mid;
    }
}

fn solve_one(input: &Vec<String>) {
    let mut highest = 0;
    for i in input {
        let mut row = MinMax { min: 0, max: 128 };
        let mut column = MinMax { min: 0, max: 8 };
        for c in i.chars() {
            match c {
                'F' => row.take_lower(),
                'B' => row.take_upper(),
                'L' => column.take_lower(),
                'R' => column.take_upper(),
                _   => panic!("character not recognised"),
            }
        }

        let seat_id = row.min * 8 + column.min;
        if seat_id > highest { highest = seat_id }
    }

    println!("solution one: {}", highest);
}

fn solve_two(input: &Vec<String>) {
    let mut seats: Vec<i32> = vec![];
    for i in input {
        let mut row = MinMax { min: 0, max: 128 };
        let mut column = MinMax { min: 0, max: 8 };
        for c in i.chars() {
            match c {
                'F' => row.take_lower(),
                'B' => row.take_upper(),
                'R' => column.take_upper(),
                'L' => column.take_lower(),
                _   => panic!("character not recognised"),
            }
        }

        let seat_id = row.min * 8 + column.min;
        seats.push(seat_id);
    }

    seats.sort();
    let mut last = seats[0] - 1;
    for s in seats {
        if s != last + 1 {
            println!("solution two: {}", last + 1);
        }
        last = s;
    }
}
