use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;
use std::collections::BTreeSet;
use std::str::FromStr;

fn main() {
    let mut vec: Vec<(BTreeSet<String>, Vec<String>)> = vec![];
    let mut signal_lengths = vec![];
    let digit_lengths: Vec<usize> = vec![6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

    if let Ok(inputs) = read_lines("input.txt") {
        println!("Processing file");
        // Consumes the iterator, returns an (Optional) String
        for line in inputs.flatten() {
            //println!("{}", line);
            let mut pattern_or_signal = line.split('|');
            let patterns: BTreeSet<String> = pattern_or_signal
                .next()
                .unwrap()
                .split_whitespace()
                .map(|pat| pat.to_string())
                .collect::<BTreeSet<String>>();
            let signals: Vec<String> = pattern_or_signal
                .next()
                .unwrap()
                .split_whitespace()
                .map(|sig| sig.chars().sorted().collect::<String>())
                .collect::<Vec<String>>();
            signals
                .iter()
                .for_each(|pat| signal_lengths.push(pat.len()));
            let entry = (patterns, signals);
            vec.push(entry);
        }
    }

    println!(
        "Part 1: {:?}",
        signal_lengths
            .iter()
            .filter(|&&len| {
                len == digit_lengths[1]
                    || len == digit_lengths[4]
                    || len == digit_lengths[7]
                    || len == digit_lengths[8]
            })
            .count()
    );

    let mut display_values = vec![];
    for (pat, sig) in vec {
        let seven = pat
            .iter()
            .filter(|p| p.len() == digit_lengths[7])
            .collect::<BTreeSet<&String>>()
            .iter()
            .next()
            .unwrap()
            .chars()
            .collect::<BTreeSet<char>>();
        let one = pat
            .iter()
            .filter(|p| p.len() == digit_lengths[1])
            .collect::<BTreeSet<&String>>()
            .iter()
            .next()
            .unwrap()
            .chars()
            .collect::<BTreeSet<char>>();
        let four = pat
            .iter()
            .filter(|p| p.len() == digit_lengths[4])
            .collect::<BTreeSet<&String>>()
            .iter()
            .next()
            .unwrap()
            .chars()
            .collect::<BTreeSet<char>>();
        let eight = pat
            .iter()
            .filter(|p| p.len() == digit_lengths[8])
            .collect::<BTreeSet<&String>>()
            .iter()
            .next()
            .unwrap()
            .chars()
            .collect::<BTreeSet<char>>();
        let l_sixes = pat
            .iter()
            .filter(|p| p.len() == 6)
            .collect::<BTreeSet<&String>>();
        let mut l_sixes_iter = l_sixes.iter();
        let mut six_int = l_sixes_iter
            .next()
            .unwrap()
            .chars()
            .collect::<BTreeSet<char>>();
        for l_six in l_sixes_iter {
            six_int = six_int
                .into_iter()
                .filter(|s| l_six.chars().collect::<BTreeSet<char>>().contains(s))
                .collect();
        }
        let l_fives = pat
            .iter()
            .filter(|p| p.len() == 5)
            .collect::<BTreeSet<&String>>();
        let mut l_fives_iter = l_fives.iter();
        let mut five_int = l_fives_iter
            .next()
            .unwrap()
            .chars()
            .collect::<BTreeSet<char>>();
        for l_five in l_fives_iter {
            five_int = five_int
                .into_iter()
                .filter(|s| l_five.chars().collect::<BTreeSet<char>>().contains(s))
                .collect();
        }
        let a = &seven - &one;
        let b = &(&four - &one).to_owned() - &five_int;
        let d = &(&four - &one).to_owned() - &b;
        let g = &(&five_int - &d).to_owned() - &a;
        let f = &(&(&six_int - &b).to_owned() - &g).to_owned() - &a;
        let c = &one - &f;
        let mut vals: String = String::new();
        for num in sig {
            match num.len() {
                2 => {
                    vals.push('1');
                }
                4 => {
                    vals.push('4');
                }
                3 => {
                    vals.push('7');
                }
                7 => {
                    vals.push('8');
                }
                5 => {
                    if !num.contains(&f.iter().next().unwrap().to_string()) {
                        vals.push('2');
                    } else if !num.contains(&c.iter().next().unwrap().to_string()) {
                        vals.push('5');
                    } else {
                        vals.push('3');
                    }
                }
                6 => {
                    if !num.contains(&d.iter().next().unwrap().to_string()) {
                        vals.push('0');
                    } else if !num.contains(&c.iter().next().unwrap().to_string()) {
                        vals.push('6');
                    } else {
                        vals.push('9');
                    }
                }
                _ => {}
            }
        }
        display_values.push(u32::from_str(&vals).unwrap());
    }
    println!("Part 2: {:?}", display_values.iter().sum::<u32>());
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
