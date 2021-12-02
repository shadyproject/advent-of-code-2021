use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let data = "input.test";
    let commands = read_input_file(data);
    part_1(&commands);
}

fn part_1(commands: &Vec<String>) {
    println!("Got {} commands", commands.len());
}

fn read_input_file(input: impl AsRef<Path>) -> Vec<String> {
    if let Ok(file) = File::open(input) {
        let commands = BufReader::new(file)
            .lines()
            .map(|l| l.expect("Could not parse input."))
            .collect::<Vec<String>>();

        return commands;
    } else {
        println!("ERRROR: Could not open input");
        return vec![];
    }
}
