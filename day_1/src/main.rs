use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut depth_change = 0;
    let mut prev = i32::MIN;
    if let Ok(file) = File::open("input") {
        println!("Opened input");
        let buffer = BufReader::new(file);

        //read the first value
        for line in buffer.lines() {
            let curr = line.unwrap().parse::<i32>().unwrap();
            if curr < prev {
                depth_change += 1;
            }

            prev = curr;
        }
    } else {
        println!("Error: Couldn't open file.")
    }

    println!("{} depth changes", depth_change);
}
