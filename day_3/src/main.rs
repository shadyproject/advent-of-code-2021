use std::{io, path::Path};

struct DiagnosticReport {
    gamma_rate: i32,
    epsilon_rate: i32,
}

impl DiagnosticReport {
    fn new() -> Self {
        Self {
            gamma_rate: 0,
            epsilon_rate: 0,
        }
    }

    fn power_consumption_rate(&self) -> i32 {
        self.gamma_rate * self.epsilon_rate
    }
}

fn main() {
    let data = "input.test";
    match read_input_file(&data) {
        Ok(commands) => {
            part_1(&commands);
            part_2(&commands);
        }
        Err(msg) => println!("{}: {}", msg, &data),
    };
}

fn part_1(commands: &Vec<String>) {
    println!("Running Part 1");
    println!("Got {} commands", commands.len());
}

fn part_2(commands: &Vec<String>) {
    println!("Running Part 2");
    println!("Got {} commands", commands.len())
}

fn read_input_file(input: impl AsRef<Path>) -> Result<Vec<String>, io::Error> {
    Ok(std::fs::read_to_string(input)?
        .lines()
        .map(|l| l.into())
        .collect::<Vec<String>>())
}
