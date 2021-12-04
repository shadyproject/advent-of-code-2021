use std::{collections::HashMap, io, path::Path};

struct DiagnosticReport {
    gamma_rate: u32,
    epsilon_rate: u32,
    oxygen_generator_rating: u32,
    co2_scrubber_rating: u32,
}

impl DiagnosticReport {
    fn new() -> Self {
        Self {
            gamma_rate: 0,
            epsilon_rate: 0,
            oxygen_generator_rating: 0,
            co2_scrubber_rating: 0,
        }
    }

    fn power_consumption_rate(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }

    fn life_suppport_rating(&self) -> u32 {
        self.co2_scrubber_rating * self.oxygen_generator_rating
    }
}

fn main() {
    let data = "input";
    println!("Using data file {}", data);
    match read_input_file(&data) {
        Ok(commands) => {
            part_2(&commands);
            let trans = transpose(commands);
            part_1(&trans);
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

fn part_2(original: &Vec<Vec<u32>>) {
    println!("Running Part 2");
    let mut r = DiagnosticReport::new();
    r.oxygen_generator_rating = calculate_o2_rating(original);
    r.co2_scrubber_rating = calculate_co2_rating(original);

    println!("O2 Rating: {}", r.oxygen_generator_rating);
    println!("CO2 Rating: {}", r.co2_scrubber_rating);
    println!("Life Support Rating: {}", r.life_suppport_rating())
}

fn calculate_co2_rating(vec: &Vec<Vec<u32>>) -> u32 {
    let mut working_set = vec.clone();
    for idx in 0..vec.first().unwrap().len() {
        if working_set.len() == 1 {
            break;
        }

        // println!("Readings: {:?}", working_set);

        let trans = transpose(working_set.clone());
        // println!("Transposed Slice: {:?}", trans.iter().nth(idx));

        let mut co2_criteria = 0;
        if !does_need_stupid_edge_case(trans.iter().nth(idx).unwrap()) {
            co2_criteria = anti_mode(trans.iter().nth(idx).unwrap()).unwrap();
        }
        // println!("CO2 Criteria: {}", co2_criteria);

        let candidates = working_set
            .iter()
            .filter(|v| does_match_criteria_at_position(co2_criteria, idx, v))
            .map(|v| v.clone())
            .collect::<Vec<Vec<u32>>>();

        working_set = candidates;
    }

    // println!("{:?}", working_set);
    to_u32(working_set.first().unwrap())
}

fn calculate_o2_rating(vec: &Vec<Vec<u32>>) -> u32 {
    //gave up fighting with the borrow checker so just clone everything lol
    let mut working_set = vec.clone();
    for idx in 0..vec.first().unwrap().len() {
        if working_set.len() == 1 {
            break;
        }

        let trans = transpose(working_set.clone());

        let mut o2_criteria = 1;
        if !does_need_stupid_edge_case(trans.iter().nth(idx).unwrap()) {
            o2_criteria = mode(trans.iter().nth(idx).unwrap()).unwrap();
        }

        let candidates = working_set
            .iter()
            .filter(|v| does_match_criteria_at_position(o2_criteria, idx, v))
            .map(|v| v.clone())
            .collect::<Vec<Vec<u32>>>();

        working_set = candidates;
    }

    to_u32(working_set.first().unwrap())
}

fn does_match_criteria_at_position(criteria: u32, position: usize, vec: &Vec<u32>) -> bool {
    vec.iter().nth(position).eq(&Some(&criteria))
}

fn does_need_stupid_edge_case(vec: &Vec<u32>) -> bool {
    let num_0 = vec.iter().filter(|n| n == &&(0 as u32)).count();
    let num_1 = vec.iter().filter(|n| n == &&(1 as u32)).count();

    num_0 == num_1
}

fn to_u32(vec: &Vec<u32>) -> u32 {
    vec.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

// https://math.stackexchange.com/questions/3446624/is-there-an-accepted-term-for-the-opposite-of-mode-in-statistics
fn anti_mode(numbers: &[u32]) -> Option<u32> {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .min_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
}

fn mode(numbers: &[u32]) -> Option<u32> {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
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
