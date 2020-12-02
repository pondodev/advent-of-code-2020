use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input").expect("failed to open file");
    let lines = io::BufReader::new(f).lines();

    let mut nums: Vec<i32> = vec![];
    for line in lines {
        nums.push(
            str::parse(line.expect("failed to read line")
                .as_str())
                .expect("failed to parse line"));
    }

    solve_one(&nums);
    solve_two(&nums);
}

fn solve_one(nums: &Vec<i32>) {
    for a in nums {
        for b in nums {
            if a + b != 2020 { continue; }

            println!("solution one is: {}", a * b);
            return
        }
    }
}

fn solve_two(nums: &Vec<i32>) {
    for a in nums {
        for b in nums {
            for c in nums {
                if a + b + c != 2020 { continue; }

                println!("solution two is: {}", a * b * c);
                return
            }
        }
    }
}
