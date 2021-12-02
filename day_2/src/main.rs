use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Position {
    depth: i32, // how deep we are
    place: i32, // how far we are from the dock (origin) aka horizontal position
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            depth: 0,
            place: 0,
            aim: 0,
        }
    }

    fn horizontal_position(&self) -> i32 {
        self.depth * self.place
    }
}

pub fn part1(commands: &Vec<String>) {
    let mut p = Position::new();
    for line in commands {
        let commands = line.split_whitespace().collect::<Vec<&str>>();
        let direction = commands[0];
        let distance = commands[1].parse::<i32>().unwrap();

        match direction {
            "forward" => p.place += distance,
            "down" => p.depth += distance,
            "up" => p.depth -= distance,
            _ => (),
        }
    }

    println!("Final horizontal position: {}", p.horizontal_position())
}

pub fn part2(commands: &Vec<String>) {
    let mut p = Position::new();
    for line in commands {
        let commands = line.split_whitespace().collect::<Vec<&str>>();
        let direction = commands[0];
        let distance = commands[1].parse::<i32>().unwrap();

        match direction {
            "forward" => {
                p.place += distance;
                p.depth += p.aim * distance;
            }
            "down" => p.aim += distance,
            "up" => p.aim -= distance,
            _ => (),
        }
    }

    println!("Final horizontal position: {}", p.horizontal_position())
}

fn main() {
    let input = "input";
    if let Ok(file) = File::open(input) {
        let buffer = BufReader::new(file);
        let commands = buffer
            .lines()
            .map(|l| l.expect("Could not parse command"))
            .collect::<Vec<String>>();

        println!("Running Part 1");
        part1(&commands);

        println!("Running Part 2");
        part2(&commands);
    } else {
        println!("Error: Unable to open input file {}", input);
    }
}
