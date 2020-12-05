use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

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
            record = format!("{} {}", record, l);
        }
    }

    if record != "" { input.push(String::from(&record[1..])) }

    solve_one(&input);
    solve_two(&input);
}

fn solve_one(input: &Vec<String>) {
    let required = vec![
        "byr:",
        "iyr:",
        "eyr:",
        "hgt:",
        "hcl:",
        "ecl:",
        "pid:"
    ];

    let mut count = 0;
    for i in input {
        if validate_passport(&i, &required) { count += 1 };
    }

    println!("solution one: {}", count);
}

fn validate_passport(passport: &String, required: &Vec<&str>) -> bool {
    for r in required {
        if !passport.contains(r) {
            return false;
        }
    }

    true
}

fn solve_two(input: &Vec<String>) {
    let required = vec![
        "byr:",
        "iyr:",
        "eyr:",
        "hgt:",
        "hcl:",
        "ecl:",
        "pid:"
    ];

    let mut valid = 0;
    for i in input {
        let fields = i.split(' ').collect::<Vec<&str>>();
        if !validate_passport(&i, &required) { continue; }
        for data in fields {
            let field = data.split(':').collect::<Vec<&str>>();
            if !validate_field(&field[0], &field[1]) {
                valid -= 1; // offset the valid count so this invalid record isn't counted
                break;
            }
        }

        valid += 1;
    }

    println!("solution two: {}", valid);
}

fn validate_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => {
            let value = value.parse::<u32>().expect("failed to parse byr");
            value >= 1920 && value <= 2002
        },
        "iyr" => {
            let value = value.parse::<u32>().expect("failed to parse iyr");
            value >= 2010 && value <= 2020
        },
        "eyr" => {
            let value = value.parse::<u32>().expect("failed to parse eyr");
            value >= 2020 && value <= 2030
        },
        "hgt" => {
            if !value.contains("in") && !value.contains("cm") { return false }
            let unit = &value[value.len() - 2..];
            let value = value[..value.len() - 2]
                .parse::<u32>().expect("failed to parse hgt");
            match unit {
                "in" => value >= 59 && value <= 76,
                "cm" => value >= 150 && value <= 193,
                _    => false,
            }
        },
       "hcl" => {
           let rgx = Regex::new(r"^#[A-Fa-f0-9]{6}$").expect("failed to parse hcl regex");
           rgx.is_match(value)
        },
        "ecl" => {
            let accepted = [
                "amb",
                "blu",
                "brn",
                "gry",
                "grn",
                "hzl",
                "oth"
            ];

            accepted.contains(&value)
        },
        "pid" => {
            let rgx = Regex::new(r"^[0-9]{9}$").expect("failed to parse pid regex");
            rgx.is_match(value)
        },
        "cid" => true,
        _     => panic!("field not recognised"),
    }
}
