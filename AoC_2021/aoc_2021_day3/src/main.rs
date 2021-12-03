use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::BTreeMap;
// use itertools::Itertools;

fn main() {

    let mut vec: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    let mut bit_freqs = BTreeMap::<usize, (i32, i32)>::new();

    if let Ok(inputs) = read_lines("input.txt") {
        println!("Processing file");
        // Consumes the iterator, returns an (Optional) String
        for input in inputs {
            if let Ok(line) = input {
                //println!("{}", ip);
                vec.push(line.chars().collect());
                for (i, bit) in line.chars().enumerate() {
                    match bit {
                        '0' => {bit_freqs.entry(i).or_insert((0,0)).0 += 1;}
                        '1' => {bit_freqs.entry(i).or_insert((0,0)).1 += 1;}
                        _ => {}
                    }
                }
            }
        }
    }

    let bit_len = bit_freqs.len();

    let mut gamma  = 0;
    let mut epsilon = 0;
    for (bit, (zeroes, ones)) in bit_freqs.iter() {
        if ones > zeroes {
            gamma += usize::pow(2, (bit_len-bit-1).try_into().unwrap());
        }
        else {
            epsilon += usize::pow(2, (bit_len-bit-1).try_into().unwrap());
        }
    }

    println!("Part 1: {}", epsilon * gamma);
    
    let mut oxygens = vec.clone();
    let mut co2s = vec.clone();
    for i in 0..bit_len {
        if oxygens.len() > 1 {
            let ones = oxygens.iter().filter(|b| b[i] == '1').collect::<Vec<_>>().len();
            if (ones as f32) >= oxygens.len() as f32 / 2 as f32 {
                oxygens = oxygens.into_iter().filter(|b| b[i] == '1').collect();
            }
            else {
                oxygens = oxygens.into_iter().filter(|b| b[i] == '0').collect();
            }
        }
        if co2s.len() > 1 {
            let ones = co2s.iter().filter(|b| b[i] == '1').collect::<Vec<_>>().len();
            if (ones as f32) < co2s.len() as f32 / 2 as f32 && ones != 0 {
                co2s = co2s.into_iter().filter(|b| b[i] == '1').collect();
            }
            else {
                co2s = co2s.into_iter().filter(|b| b[i] == '0').collect();
            }
        }
    }

    let mut o = 0;
    for (i, b) in oxygens[0].iter().rev().enumerate() {
        if *b == '1' {
            o += usize::pow(2,i.try_into().unwrap());
        }
    }
    let mut c = 0;
    for (i, b) in co2s[0].iter().rev().enumerate() {
        if *b == '1' {
            c += usize::pow(2,i.try_into().unwrap());
        }
    }
    
    println!("Part 2: {}", o * c);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}