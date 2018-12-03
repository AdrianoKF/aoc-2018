extern crate common;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

use common::aoc::util;

#[cfg(test)]
mod tests {
    mod counts {
        use count;
        use Counts;

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

    mod distance {
        use distance;

        #[test]
        fn distance_equal() {
            assert_eq!(0, distance("abcdef", "abcdef"));
        }

        #[test]
        fn distance_one() {
            assert_eq!(1, distance("abcdef", "abxdef"));
        }

        #[test]
        fn distance_two() {
            assert_eq!(2, distance("abcdef", "abxyef"));
        }
    }
}

/// Read the problem set line by line
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

/// Calculate number of doubles and triples (repeated characters) in `word`
fn count(word: &str) -> Counts {
    let mut freq = [0usize; 26];

    // Count the frequency of each character
    for ch in word.chars() {
        let ascii = ch as u8;
        let idx = (ascii - 'a' as u8) as usize;
        freq[idx] += 1;
    }
    Counts {
        word: word.to_string(),
        doubles: freq.iter().filter(|&&c| c == 2).count(),
        triples: freq.iter().filter(|&&c| c == 3).count(),
    }
}

/// Calculate number of different characters between `a` and `b`
fn distance(a: &str, b: &str) -> usize {
    assert_eq!(a.len(), b.len());
    a.chars().zip(b.chars()).map(|(c1, c2)| if c1 == c2 { 0 } else { 1 }).sum()
}

/// Find a pair of words in `data` that only differs in a single position
fn find_candidates(data: Vec<String>) -> Option<(String, String)> {
    for word in data.clone() {
        match data.clone().iter().find(|&w2| distance(word.as_str(), w2.as_str()) == 1) {
            Some(other) => return Some((word, other.clone())),
            _ => {}
        }
    }
    None
}

/// Solution for part 1 -> checksum calculation
fn part_01() -> Result<usize> {
    let data = read_input()?;
    let counts: Vec<Counts> = data.iter().map(|d| count(d.as_str())).collect();
    let double_count = counts.iter().filter(|c| c.doubles > 0).count();
    let triple_count = counts.iter().filter(|c| c.triples > 0).count();

    Ok(double_count * triple_count)
}

/// Solution for part 2 -> derive correct ID
fn part_02() -> Option<String> {
    let data = read_input().ok()?;
    match find_candidates(data) {
        Some((w1, w2)) => {
            println!("[Part 2] Candidate pair: {} <--> {}", w1, w2);
            let solution: String = w1.chars().zip(w2.chars())
                .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
                .collect();
            Some(solution)
        }
        _ => None
    }
}

fn main() -> Result<()> {
    println!("[Part 1] Checksum: {}", part_01()?);
    println!("[Part 2] Solution ID: {}", part_02().expect("No solution for part 2"));
    Ok(())
}
