#![feature(stdin_forwarders)]
use std::{
    error::Error,
    fs::File,
    io::{stdin, BufRead, BufReader},
};

fn load() -> Result<Vec<String>, Box<dyn Error>> {
    let f = File::open("/usr/share/dict/words")?;
    let f = BufReader::new(f);
    let mut words = vec![];
    let mut counts = [[0u64; 256]; 5];
    for line in f.lines() {
        let line = line?;
        if line.len() == 5
            && line.is_ascii()
            && line
                .as_bytes()
                .iter()
                .all(|x| matches!(*x as char, 'a'..='z'))
        {
            words.push(line.to_string());
            for i in 0..5 {
                counts[i][line.as_bytes()[i] as usize] += 1;
            }
        }
    }
    Ok(words)
}

#[derive(Debug)]
struct Matches {
    // The characters in `any` must exist somewhere in the target word
    any: Vec<u8>,
    // The characters in `never` must not exist anywhere in the target word
    never: Vec<u8>,
    // The (position, character) tuples in exact must exist in that exact
    // location
    exact: Vec<(usize, u8)>,
    // The (position, character) tuples in never must not exist in that
    // exact position (because they came up as yellow in that position)
    never_exact: Vec<(usize, u8)>,
}

fn matches_any(c: u8, s: &str) -> bool {
    for t in s.as_bytes() {
        if c == *t {
            return true;
        }
    }
    false
}

impl Matches {
    fn matches(&self, s: &str) -> bool {
        for (i, c) in s.as_bytes().iter().enumerate() {
            if self.never.iter().any(|v| v == c) {
                println!("eliminating {} for never {:?}", s, (i, *c as char));
                return false;
            }
            if self.never_exact.iter().any(|(j, v)| (*j, v) == (i, c)) {
                println!("eliminating {} for never_exact", s);
                return false;
            }
        }

        for (i, c) in &self.exact {
            if s.as_bytes()[*i] != *c {
                println!("eliminating {} for exact {:?}", s, (i, *c as char));
                return false;
            }
        }

        for c in &self.any {
            if !matches_any(*c, s) {
                println!("eliminating {} for any {}", s, *c as char);
                return false;
            }
        }

        true
    }

    fn from(s: &str, target: &str) -> Matches {
        assert_eq!(s.len(), 5);
        assert_eq!(target.len(), 5);
        let mut m = Matches {
            any: vec![],
            never_exact: vec![],
            exact: vec![],
            never: vec![],
        };

        for (i, (&c, &t)) in s
            .as_bytes()
            .iter()
            .zip(target.as_bytes().iter())
            .enumerate()
        {
            if c == t {
                // green
                m.exact.push((i, c));
            } else if matches_any(c, target) {
                // yellow
                m.any.push(c);
                m.never_exact.push((i, c));
            } else {
                // grey
                m.never.push(c);
            }
        }
        m
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let words = load()?;
    let mut lines_iter = stdin().lines();
    let target = lines_iter.next().unwrap()?;
    let guesses: Result<Vec<String>, _> = lines_iter.collect();
    let guesses = guesses?;
    process(&words, &target, guesses.as_ref());
    Ok(())
}

fn process(words: &[String], target: &str, guesses: &[String]) {
    let mut all_matches: Vec<Matches> = vec![];
    for guess in guesses {
        let m = Matches::from(&guess, &target);
        println!("{:?}", m);
        all_matches.push(m);
        let possible: Vec<&String> = words
            .iter()
            .filter(|w| {
                for m in &all_matches {
                    if !m.matches(w) {
                        return false;
                    }
                }
                true
            })
            .collect();
        for p in &possible {
            println!("possible: {}", p);
        }
        println!("total possible: {}", possible.len());
    }
}
