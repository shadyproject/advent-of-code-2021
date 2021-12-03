use std::{collections::HashMap, io, path::Path};

struct DiagnosticReport {
    gamma_rate: u32,
    epsilon_rate: u32,
}

impl DiagnosticReport {
    fn new() -> Self {
        Self {
            gamma_rate: 0,
            epsilon_rate: 0,
        }
    }

    fn power_consumption_rate(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }
}

fn main() {
    let data = "input";
    match read_input_file(&data) {
        Ok(commands) => {
            let trans = transpose(commands);
            part_1(&trans);
            part_2(&trans);
        }
        Err(msg) => println!("{}: {}", msg, &data),
    };
}

fn part_1(commands: &Vec<Vec<u32>>) {
    println!("Running Part 1");
    let mut r = DiagnosticReport::new();
    let gamma_bits = commands
        .iter()
        .map(|b| mode(b).unwrap())
        .collect::<Vec<u32>>();
    r.gamma_rate = to_u32(&gamma_bits);
    // there is for sure a better way to do this
    let inverse = gamma_bits
        .iter()
        .map(|b| if *b == 0 { 1 } else { 0 })
        .collect::<Vec<u32>>();
    r.epsilon_rate = to_u32(&inverse);
    println!("Gamma Rate: {}", r.gamma_rate);
    println!("Epsilon Rate: {}", r.epsilon_rate);
    println!("Power Consumption: {}", r.power_consumption_rate());
}

fn part_2(commands: &Vec<Vec<u32>>) {
    println!("Running Part 2");
    println!("{:?}", commands);
}

fn to_u32(vec: &Vec<u32>) -> u32 {
    vec.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn mode(numbers: &[u32]) -> Option<u32> {
    let mut counts = HashMap::new();

    numbers.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn read_input_file(input: impl AsRef<Path>) -> Result<Vec<Vec<u32>>, io::Error> {
    Ok(std::fs::read_to_string(input)?
        .lines()
        .map(|l| l.to_string())
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>())
}
