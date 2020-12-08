use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input").expect("failed to open file");
    let lines = io::BufReader::new(f).lines();

    let input: Vec<String> = lines.map(|line| line.expect("failed to read line")).collect();

    solve_one(&input);
    solve_two(&input);
}

struct Instruction {
    opcode: String,
    argument: i32,
    executed: bool,
}

impl Instruction {
    fn new(opcode: &str, argument: &str) -> Self {
        let argument = match &argument[..1] {
            "+" => *&argument[1..].parse::<i32>().expect("failed to parse argument to +i32"),
            "-" => &argument[1..].parse::<i32>().expect("failed to parse argument to -i32") * -1,
            _   => panic!("failed to parse argument"),
        };
        Instruction {
            opcode: String::from(opcode),
            argument,
            executed: false,
        }
    }
}

struct Core {
    acc: i32,
    pc: usize,
    insts: Vec<Instruction>,
}

impl Core {
    fn new(insts: &Vec<String>) -> Self {
        let insts = insts.iter().map(|i| {
            let parts = i.split(' ').collect::<Vec<&str>>();
            Instruction::new(parts[0], parts[1])
        }).collect();
        Core {
            acc: 0,
            pc: 0,
            insts,
        }
    }

    fn run(&mut self) -> Result<i32, i32> {
        loop {
            // return if we're at the end of the program
            if self.pc >= self.insts.len() { return Ok(self.acc) }

            // return if we've already executed this instruction
            if self.insts[self.pc].executed { return Err(self.acc) }
            else { self.insts[self.pc].executed = true }

            match self.insts[self.pc].opcode.as_str() {
                "nop" => (),
                "acc" => self.acc += self.insts[self.pc].argument,
                "jmp" => {
                    self.pc = (self.pc as i32 + self.insts[self.pc].argument) as usize; // ew
                    continue // we set the pc explicitly, so we can skip incrementing it at the end
                },
                _ => panic!("unrecognised opcode"),
            }

            self.pc += 1;
        }
    }
}

fn solve_one(input: &Vec<String>) {
    let mut core = Core::new(&input);
    let answer = match core.run() {
        Ok(_) => panic!("uhhhh this shouldn't happen"),
        Err(a) => a,
    };
    println!("solution one: {}", answer);
}

fn solve_two(input: &Vec<String>) {
    let insts = input.iter().map(|i| {
        let parts = i.split(' ').collect::<Vec<&str>>();
        Instruction::new(parts[0], parts[1])
    }).collect::<Vec<Instruction>>();

    for i in 0..insts.len() {
        if insts[i].opcode == "acc" { continue } // all acc instructions are correct
        let mut core = Core::new(&input);

        // do the switcheroo
        match insts[i].opcode.as_str() {
            "nop" => core.insts[i].opcode = String::from("jmp"),
            "jmp" => core.insts[i].opcode = String::from("nop"),
            _     => panic!("failed to switch instructions"),
        }

        match core.run() {
            Ok(a) => {
                println!("solution two: {}", core.acc);
                return
            },
            Err(_) => (),
        }
    }

    panic!("failed to find answer");
}
