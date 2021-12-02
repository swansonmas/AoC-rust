use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// use itertools::Itertools;

fn main() {

    let mut sub_cmds: Vec<(String,i32)> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        println!("Processing file");
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut ip_split = ip.split_whitespace();
                let command = ip_split.next().unwrap().to_string();
                let value = ip_split.next().unwrap().parse::<i32>().unwrap();
                sub_cmds.push((command, value));
            }
        }
    }
    let mut location = (0,0);
    for (dir, val) in sub_cmds.iter() {
        match dir.as_str() {
            "forward" => location.0 += val,
            "down" => location.1 += val,
            "up" => location.1 -= val,
            _ => {}
        }
    }

    println!("Part 1: {}", location.0 * location.1);

    let mut loc_aim = (0,0,0);
    for (dir, val) in sub_cmds.iter() {
        match dir.as_str() {
            "forward" => {
                loc_aim.0 += val;
                loc_aim.1 += val*loc_aim.2;
            },
            "down" => loc_aim.2 += val,
            "up" => loc_aim.2 -= val,
            _ => {}
        }
    }

    println!("Part 2: {}", loc_aim.0 * loc_aim.1);
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}