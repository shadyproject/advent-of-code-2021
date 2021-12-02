use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let data = "input.test";
    match read_input_file(data) {
        Ok(commands) => part_1(&commands),
        Err(msg) => println!("{}", msg),
    };
}

fn part_1(commands: &Vec<String>) {
    println!("Got {} commands", commands.len());
}

fn read_input_file(input: impl AsRef<Path>) -> Result<Vec<String>, &'static str> {
    if let Ok(file) = File::open(input) {
        let commands = BufReader::new(file)
            .lines()
            .map(|l| l.expect("Could not parse input."))
            .collect::<Vec<String>>();

        return Ok(commands);
    } else {
        return Err("ERROR: Could not open input");
    }
}
