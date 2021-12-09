use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Vent {
    x: usize,
    y: usize,
    height: u32,
}

fn get_neighbors(vent_grid: &[Vec<Vent>], x_idx: usize, y_idx: usize) -> Vec<Vent> {
    let mut neighbors: Vec<Vent> = vec![];
    let mut x_offs = vec![];
    let mut y_offs = vec![];
    if x_idx != 0 {
        x_offs.push(-1);
    }
    if x_idx != vent_grid[0].len() - 1 {
        x_offs.push(1);
    }
    for x_offset in x_offs.iter() {
        let neighbor_x = (x_idx as isize + x_offset) as usize;
        neighbors.push(vent_grid[y_idx][neighbor_x]);
    }
    if y_idx != 0 {
        y_offs.push(-1);
    }
    if y_idx != vent_grid.len() - 1 {
        y_offs.push(1);
    }
    for y_offset in y_offs.iter() {
        let neighbor_y = (y_idx as isize + y_offset) as usize;
        neighbors.push(vent_grid[neighbor_y][x_idx]);
    }

    neighbors
}

fn find_low_points(vent_grid: &[Vec<Vent>]) -> Vec<Vent> {
    let mut low_spots: Vec<Vent> = vec![];
    for y_idx in 0..vent_grid.len() {
        for x_idx in 0..vent_grid[y_idx].len() {
            if vent_grid[y_idx][x_idx].height
                < get_neighbors(vent_grid, x_idx, y_idx)
                    .iter()
                    .map(|v| v.height)
                    .min()
                    .unwrap()
            {
                low_spots.push(vent_grid[y_idx][x_idx]);
            }
        }
    }

    low_spots
}

fn find_basin(height_grid: &[Vec<Vent>], low_point: &Vent) -> HashSet<Vent> {
    let mut basin: HashSet<Vent> = HashSet::new();
    let mut prev_size = 0;
    basin.insert(*low_point);
    while prev_size != basin.len() {
        let mut new_vents: HashSet<Vent> = HashSet::new();
        prev_size = basin.len();
        for b_vent in basin.iter() {
            new_vents.extend(
                get_neighbors(height_grid, b_vent.x, b_vent.y)
                    .iter()
                    .filter(|v| v.height != 9),
            );
        }
        basin.extend(new_vents);
    }

    basin
}

fn main() {
    let mut vec: Vec<_> = Vec::new();
    if let Ok(inputs) = read_lines("input.txt") {
        println!("Processing file");
        // Consumes the iterator, returns an (Optional) String
        for (y, line) in inputs.flatten().enumerate() {
            //println!("{}", line);
            vec.push(
                line.chars()
                    .enumerate()
                    .map(|(x, h)| Vent {
                        x,
                        y,
                        height: u32::from_str(&h.to_string()).unwrap(),
                    })
                    .collect::<Vec<Vent>>(),
            );
        }
    }
    let low_points = find_low_points(&vec);

    println!(
        "Part 1: {:?}",
        low_points.iter().fold(0, |acc, pt| acc + pt.height + 1)
    );

    let mut basin_sizes = low_points
        .iter()
        .map(|lp| find_basin(&vec, lp).len())
        .collect::<Vec<usize>>();
    basin_sizes.sort_unstable();

    println!(
        "Part 2: {:?}",
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
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
