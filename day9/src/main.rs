use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let f = File::open("input").expect("failed to open file");
    let lines = io::BufReader::new(f).lines();

    let input: VecDeque<u64> = lines.map(|line| {
        line.expect("failed to read line")
            .parse().expect("failed to parse line")
    }).collect();

    solve_one(&input);
    solve_two(&input);
}

fn find_anomaly(mut input: VecDeque<u64>) -> Result<u64, ()> {
    let mut working_queue: VecDeque<u64> = VecDeque::new();
    for _ in 0..25 {
        working_queue.push_back(
            input.pop_front().expect("failed to pop input"));
    }

    while input.len() != 0 {
        let current = input.pop_front().expect("failed to pop input");
        let mut pass = false;
        for a in working_queue.iter() {
            for b in working_queue.iter() {
                if a == b { continue } // can't add the same number together
                if a + b == current { pass = true }
            }
        }

        if !pass { // found the anomaly
            return Ok(current)
        } else { // continue on
            working_queue.push_back(current);
            working_queue.pop_front();
        }
    }

    Err(())
}

fn solve_one(input: &VecDeque<u64>) {
    let anomaly = find_anomaly(input.clone()).expect("failed to find anomaly");
    println!("solution one: {}", anomaly);
}

fn solve_two(input: &VecDeque<u64>) {
    let anomaly = find_anomaly(input.clone()).expect("failed to find anomaly");
    let mut working_queue: VecDeque<u64> = VecDeque::new();
    let mut chain_length = 2;

    while chain_length != input.len() { // we know we've failed if we've gotten this far
        // start by prepping the working queue and numbers queue
        let mut numbers = input.clone();
        working_queue.clear();
        for _ in 0..chain_length {
            working_queue.push_back(
                numbers.pop_front().expect("failed to pop number"));
        }

        // now we start searching
        loop {
            let mut sum = 0;
            for num in working_queue.iter() { sum += num }

            if sum == anomaly { // we have found our sequence!
                let mut min = working_queue[0];
                let mut max = working_queue[0];
                for num in working_queue.iter() {
                    if *num < min { min = *num }
                    if *num > max { max = *num }
                }
                println!("solution two: {}", min + max);
                return
            } else { // we have not found our sequence, continue
                if numbers.len() == 0 { break } // break out if we've consumed all numbers

                working_queue.push_back(
                    numbers.pop_front().expect("failed to pop number"));
                working_queue.pop_front();
            }
        }

        chain_length += 1; // add to the chain if we can't find it
    }

    panic!("failed to find sequence");
}
