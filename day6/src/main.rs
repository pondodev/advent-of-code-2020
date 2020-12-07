#[allow(dead_code, unused_imports)]
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input").expect("failed to open file");
    let lines = io::BufReader::new(f).lines();

    let mut input: Vec<String> = vec![];
    let mut record = String::new();
    for line in lines {
        let l = line.expect("failed to read line");
        if l == "" {
            input.push(String::from(&record[1..]));
            record = String::new();
        } else {
            record = format!("{}/{}", record, l);
        }
    }

    if record != "" { input.push(String::from(&record[1..])) }

    solve_one(&input);
    solve_two(&input);
}

fn solve_one(input: &Vec<String>) {
    let mut count = 0;

    for i in input {
        let mut flags: u32 = 0b0;
        for c in i.chars() {
            let ascii_code = c as u32;
            // don't parse anything that isn't an accepted character
            if ascii_code < 97 || ascii_code > 122 { continue }

            // get how many times we should shift the bit left, and shift it
            let shift = ascii_code - 97;
            flags |= 0b1 << shift;
        }

        // now we can count all the yes answers
        count += flags.count_ones();
    }

    println!("solution one: {}", count);
}

fn solve_two(input: &Vec<String>) {
    let mut count = 0;

    for i in input {
        let mut group_flags: Vec<u32> = vec![];
        let mut flags: u32 = 0b0;
        for c in i.chars() {
            // we use a slash to delimit people in a group
            if c == '/' {
                group_flags.push(flags);
                flags = 0b0;
                continue
            }

            let ascii_code = c as u32;
            // don't parse anything that isn't an accepted character
            if ascii_code < 97 || ascii_code > 122 { continue }

            // get how many times we should shift the bit left, and shift it
            let shift = ascii_code - 97;
            flags |= 0b1 << shift;
        }

        // now we can bitwise and all of the flags together to see which ones everyone answered yes on
        while group_flags.len() != 0 {
            flags &= group_flags.pop().expect("vector was empty");
        }

        count += flags.count_ones();
    }

    println!("solution two: {}", count);
}
