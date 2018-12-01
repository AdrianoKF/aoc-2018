use std::collections::BTreeSet;
use std::io::prelude::*;
use std::io::Result;

use std::env;
use std::fs::File;

/// Parse the input file and return the entries as a vector.
///
/// Input contains one entry per line, each a positive or negative integer.
///
/// ## Example input
/// ```
/// +1
/// +2
/// -2
/// -1
/// ```
fn read_input(args: &[String]) -> Result<Vec<i32>> {
    let filename = args[1].clone();
    println!("Using input file: {}", filename);

    let mut f = File::open(filename).expect("Input file not readable");
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut result: Vec<i32> = Vec::new();
    for token in buf.split('\n') {
        if !token.is_empty() {
            let val: i32 = token.parse().expect("Invalid token!");
            result.push(val);
        }
    }
    Ok(result)
}

/// Solution for the first part: Simple tally
fn part_01(data: &[i32]) -> i32 {
    let mut tally: i32 = 0;
    for d in data {
        tally += d;
    }
    tally
}

/// Solution for the second part: Find first repeated frequency
fn part_02(data: &[i32]) -> i32 {
    let mut tally: i32 = 0;
    let mut history: BTreeSet<i32> = BTreeSet::new();

    'out: loop {
        for d in data {
            tally += d;

            if history.contains(&tally) {
                break 'out;
            }
            history.insert(tally);
        }
    }
    tally
}

/// Solution for [Advent of Code 2018, Day 01](https://adventofcode.com/2018/day/1)
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let data = read_input(&args)?;

    let tally: i32 = match args[2].as_str() {
        "1" => Ok(part_01(&data)),
        "2" => Ok(part_02(&data)),
        _ => Err(()),
    }.expect("Invalid part");

    println!("Tally: {}", tally);
    Ok(())
}
