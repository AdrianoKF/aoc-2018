extern crate common;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

use common::aoc::util;

#[cfg(test)]
mod tests {
    use Counts;
    use count;

    #[test]
    fn count_none() {
        let word = "abcdef";
        assert_eq!(Counts { word: word.to_string(), doubles: 0, triples: 0 }, count(word));
    }

    #[test]
    fn count_double() {
        let word = "abbcde";
        assert_eq!(Counts { word: word.to_string(), doubles: 1, triples: 0 }, count(word));
    }

    #[test]
    fn count_triple() {
        let word = "aabacd";
        assert_eq!(Counts { word: word.to_string(), doubles: 0, triples: 1 }, count(word));
    }

    #[test]
    fn count_both() {
        let word = "bababc";
        assert_eq!(Counts { word: word.to_string(), doubles: 1, triples: 1 }, count(word));
    }
}

fn read_input() -> Result<Vec<String>> {
    let filename = util::get_filename()?;
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf.lines().map(&str::to_string).collect())
}

#[derive(PartialEq, Eq, Debug)]
struct Counts {
    word: String,
    doubles: usize,
    triples: usize,
}

fn count(data: &str) -> Counts {
    let mut freq = [0usize; 26];

    // Count the frequency of each character
    for ch in data.chars() {
        let ascii = ch as u8;
        let idx = (ascii - 'a' as u8) as usize;
        freq[idx] += 1;
    }
    Counts {
        word: data.to_string(),
        doubles: freq.iter().filter(|&&c| c == 2).count(),
        triples: freq.iter().filter(|&&c| c == 3).count(),
    }
}

fn part_01() -> Result<usize> {
    let data = read_input()?;
    let counts: Vec<Counts> = data.iter().map(|d| count(d.as_str())).collect();
    let double_count = counts.iter().filter(|c| c.doubles > 0).count();
    let triple_count = counts.iter().filter(|c| c.triples > 0).count();

    Ok(double_count * triple_count)
}

fn main() -> Result<()> {
    println!("[Part 1] Checksum: {}", part_01()?);
    Ok(())
}
