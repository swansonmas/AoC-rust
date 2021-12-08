use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// use itertools::Itertools;

fn main() {
    let mut crabs: Vec<_> = Vec::new();

    if let Ok(inputs) = read_lines("input.txt") {
        for line in inputs.flatten() {
            //println!("{}", line);
            crabs = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        }
    }
    crabs.sort_unstable();
    let median = crabs[crabs.len() / 2];
    let mut cnst_fuel_costs = 0;
    for crab in crabs.iter() {
        cnst_fuel_costs += (*crab - median).abs();
    }

    println!("Part 1: {}", cnst_fuel_costs);

    let avg = (crabs.iter().sum::<i32>() as f32 / crabs.len() as f32).floor() as i32;
    let mut lin_fuel_costs = 0;
    for crab in crabs.iter() {
        lin_fuel_costs += (0..(*crab - avg).abs() + 1).sum::<i32>();
    }

    println!("Part 2: {}", lin_fuel_costs);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
