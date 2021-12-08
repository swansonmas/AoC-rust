use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// use itertools::Itertools;

fn main() {
    let mut lant_fish = [0usize; 9];
    let lant_spawn_time = 6;
    let lant_new_time = 2;

    if let Ok(inputs) = read_lines("input.txt") {
        for line in inputs.flatten() {
            let init_fish: Vec<_> = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            for fish in init_fish.iter() {
                lant_fish[*fish] += 1;
            }
        }
    }

    for _ in 0..80 {
        let spawn_fish = lant_fish[0];
        lant_fish.copy_within(1.., 0);
        lant_fish[lant_spawn_time] += spawn_fish;
        lant_fish[lant_spawn_time + lant_new_time] = spawn_fish;
    }
    println!("Part 1: {:?}", lant_fish.iter().sum::<usize>());

    for _ in 0..(256 - 80) {
        let spawn_fish = lant_fish[0];
        lant_fish.copy_within(1.., 0);
        lant_fish[lant_spawn_time] += spawn_fish;
        lant_fish[lant_spawn_time + lant_new_time] = spawn_fish;
    }

    println!("Part 2: {}", lant_fish.iter().sum::<usize>());
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
