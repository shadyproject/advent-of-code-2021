use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::SplitWhitespace,
};

struct Position {
    depth: i32, // how deep we are
    place: i32, // how far we are from the dock (origin) aka horizontal position
}

impl Position {
    fn new() -> Self {
        Self { depth: 0, place: 0 }
    }

    fn horizontal_position(&self) -> i32 {
        self.depth * self.place
    }
}

pub fn part1(input: BufReader<File>) {
    let mut p = Position::new();
    for line in input.lines().map(|l| l.expect("Could not parse string")) {
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

fn main() {
    let input = "input";
    if let Ok(file) = File::open(input) {
        let buffer = BufReader::new(file);
        part1(buffer);
    } else {
        println!("Error: Unable to open input file {}", input);
    }
}
