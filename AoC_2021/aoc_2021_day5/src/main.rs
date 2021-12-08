use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;
// use itertools::Itertools;

fn main() {
    let mut indxs = vec![];
    let mut straights = vec![];
    let mut diags = vec![];

    let re = Regex::new(r"(\d*),(\d*) -> (\d*),(\d*)").unwrap();

    if let Ok(inputs) = read_lines("input.txt") {
        println!("Processing file");
        for line in inputs.flatten() {
            //println!("{}", line);
            for cap in re.captures_iter(&line) {
                let x1 = cap[1].parse::<i32>().unwrap();
                indxs.push(x1);
                let y1 = cap[2].parse::<i32>().unwrap();
                indxs.push(y1);
                let x2 = cap[3].parse::<i32>().unwrap();
                indxs.push(x2);
                let y2 = cap[4].parse::<i32>().unwrap();
                indxs.push(y2);
                if x1 == x2 || y1 == y2 {
                    straights.push((x1, y1, x2, y2));
                } else {
                    diags.push((x1, y1, x2, y2));
                }
            }
        }
    }

    let dim = indxs.iter().max().unwrap();
    let mut vents = vec![vec![0u8; *dim as usize + 1]; *dim as usize + 1];
    for coord in straights.iter() {
        let (x1, y1, x2, y2) = coord;
        let path = (*x2 - *x1, *y2 - *y1);
        if path.1 == 0 {
            // Horiz path
            if path.0 < 0 {
                for x in 0..path.0.abs() + 1 {
                    vents[(*x2 as i32 + x) as usize][*y1 as usize] += 1;
                }
            } else {
                for x in 0..path.0.abs() + 1 {
                    vents[(*x1 as i32 + x) as usize][*y1 as usize] += 1;
                }
            }
        } else if path.0 == 0 {
            // Vert path
            if path.1 < 0 {
                for y in 0..path.1.abs() + 1 {
                    vents[*x1 as usize][(*y2 as i32 + y) as usize] += 1;
                }
            } else {
                for y in 0..path.1.abs() + 1 {
                    vents[*x1 as usize][(*y1 as i32 + y) as usize] += 1;
                }
            }
        }
    }

    println!(
        "Part 1: {}",
        vents.iter().fold(0, |acc, row| acc
            + row.iter().filter(|&col| *col >= 2).count())
    );

    for diag in diags.iter() {
        let (x1, y1, x2, y2) = diag;
        let path = (*x2 - *x1, *y2 - *y1);
        let x_del = path.0 / path.0.abs();
        let y_del = path.1 / path.1.abs();
        for i in 0..path.0.abs() + 1 {
            // Diagonal
            vents[(*x1 as i32 + x_del * i) as usize][(*y1 as i32 + y_del * i) as usize] += 1;
        }
    }

    println!(
        "Part 2: {}",
        vents.iter().fold(0, |acc, row| acc
            + row.iter().filter(|&col| *col >= 2).count())
    );
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
