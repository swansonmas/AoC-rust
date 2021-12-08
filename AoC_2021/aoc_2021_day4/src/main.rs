use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::{HashMap, HashSet};
// use itertools::Itertools;

fn main() {
    let mut calls: Vec<_> = Vec::<u32>::new();
    let mut calls_to_card_bingos = HashMap::<u32, Vec<String>>::new();
    let mut boards = HashMap::<u32, Vec<u32>>::new();
    let mut bingo_counts = HashMap::<String, u32>::new();

    if let Ok(inputs) = read_lines("input.txt") {
        println!("Processing file");
        let mut board = 0;
        let mut row_indx = 0;
        for line in inputs.flatten() {
            //println!("{}", line);
            if line.is_empty() {
                board += 1;
                row_indx = 0;
            } else if calls.is_empty() {
                calls = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                for num in calls.iter() {
                    calls_to_card_bingos
                        .entry(*num)
                        .or_insert_with(Vec::<String>::new);
                }
            } else {
                boards.entry(board).or_insert_with(Vec::<u32>::new);
                bingo_counts.insert(format!("{}c{}", board, row_indx), 0);
                bingo_counts.insert(format!("{}r{}", board, row_indx), 0);
                let row: Vec<_> = line
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();
                for (col, ball) in row.iter().enumerate() {
                    boards.entry(board).and_modify(|v| v.push(*ball));
                    calls_to_card_bingos
                        .entry(*ball)
                        .and_modify(|v| v.push(format!("{}r{}", board, row_indx)));
                    calls_to_card_bingos
                        .entry(*ball)
                        .and_modify(|v| v.push(format!("{}c{}", board, col)));
                }
                row_indx += 1;
            }
        }
    }

    let mut called: Vec<u32> = Vec::<u32>::new();
    let mut won_boards: HashSet<u32> = HashSet::new();
    let mut first_win = (0, 0, Vec::<u32>::new());
    let mut last_win = (0, 0, Vec::<u32>::new());
    for num in calls.iter() {
        called.push(*num);
        let bingo_calls = calls_to_card_bingos.get(num).unwrap();
        for bingo_call in bingo_calls {
            bingo_counts
                .entry(bingo_call.to_string())
                .and_modify(|v| *v += 1);
            if *bingo_counts.get(bingo_call).unwrap() == 5 {
                let board_win = bingo_call
                    .split(&['r', 'c'][..])
                    .next()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                if won_boards.contains(&board_win) {
                    continue;
                }
                won_boards.insert(board_win);
                if first_win == (0, 0, vec![]) {
                    first_win = (board_win, *num, called.to_owned());
                } else {
                    last_win = (board_win, *num, called.to_owned());
                }
            }
        }
    }
    let won_board = boards.get_mut(&first_win.0).unwrap();
    won_board.retain(|v| !first_win.2.contains(v));

    println!("Part 1: {}", won_board.iter().sum::<u32>() * first_win.1);

    let lost_board = boards.get_mut(&last_win.0).unwrap();
    lost_board.retain(|v| !last_win.2.contains(v));

    println!("Part 2: {}", lost_board.iter().sum::<u32>() * last_win.1);
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
