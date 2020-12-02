use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input").expect("failed to open file");
    let lines = io::BufReader::new(f).lines();

    let mut input: Vec<String> = vec![];
    for line in lines {
        input.push(line.expect("failed to read line"));
    }

    solve_one(&input);
    solve_two(&input);
}

fn solve_one(input: &Vec<String>) {
    let mut valid = 0;
    for line in input {
        // split the password away from the rule set
        let split = line.split(':').collect::<Vec<&str>>();
        let rule = split[0];
        let password = &split[1][1..];

        // extract rule set
        let check = rule.chars().last().expect("failed to get last char");
        let range = &rule[..rule.len() - 2]
            .split('-').collect::<Vec<&str>>();
        let min = range[0].parse::<i32>().expect("failed to parse min");
        let max = range[1].parse::<i32>().expect("failed to parse max");

        // check password against rules
        let mut counter = 0;
        for c in password.chars() {
            if c == check { counter += 1; }
        }

        if counter >= min && counter <= max { valid += 1; }
    }

    println!("solution one: {}", valid);
}

fn solve_two(input: &Vec<String>) {
    let mut valid = 0;
    for line in input {
        // split the password away from the rule set
        let split = line.split(':').collect::<Vec<&str>>();
        let rule = split[0];
        let password = &split[1][1..];

        // extract rule set
        let check = rule.chars().last().expect("failed to get last char");
        let range = &rule[..rule.len() - 2]
            .split('-').collect::<Vec<&str>>();
        let pos1 = range[0].parse::<usize>().expect("failed to parse min");
        let pos2 = range[1].parse::<usize>().expect("failed to parse max");

        let pos1 = password.chars()
            .nth(pos1 - 1).expect("failed to extract char at pos 1");
        let pos2 = password.chars()
            .nth(pos2 - 1).expect("failed to extract char at pos 2");

        // check password against rules
        if (pos1 == check || pos2 == check) && !(pos1 == check && pos2 == check) { valid += 1; }
    }

    println!("solution two: {}", valid);
}
