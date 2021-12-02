use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut depth_change = 0;
    let mut prev = i32::MAX;

    if let Ok(file) = File::open("input") {
        println!("Opened input");
        let buffer = BufReader::new(file);

        for window in buffer
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .windows(3)
        {
            println!("{:?}", window);
            let curr: i32 = window.iter().sum();
            if curr > prev {
                depth_change += 1;
            }

            prev = curr;
        }
    } else {
        println!("Error: Couldn't open file.")
    }

    println!("{} depth changes", depth_change);
}
