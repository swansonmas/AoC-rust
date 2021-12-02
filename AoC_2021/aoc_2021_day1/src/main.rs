use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn main() {

    let mut depths: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        println!("Processing file");
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", ip);
                depths.push(ip.parse::<i32>().unwrap());
            }
        }
    }
    
    let mut count = 0;
    for (curr_depth, next_depth) in depths.iter().tuple_windows() {
        if curr_depth < next_depth {
            count += 1;
        }
    }

    println!("Part 1: {}", count);
    
    let mut summed_depths: Vec<i32> = Vec::new();
    for (depth1, depth2, depth3) in depths.iter().tuple_windows() {
        summed_depths.push(depth1 + depth2 + depth3);
    }

    count = 0;
    for (curr_depth, next_depth) in summed_depths.iter().tuple_windows() {
        if curr_depth < next_depth {
            count += 1;
        }
    }

    println!("Part 2: {}", count);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}