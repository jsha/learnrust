use std::{error::Error, fmt::Write};

fn load() -> Vec<&'static str> {
    include_str!("../wordle-words.txt").split("\n").collect()
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
                // println!("eliminating {} for never {:?}", s, (i, *c as char));
                return false;
            }
            if self.never_exact.iter().any(|(j, v)| (*j, v) == (i, c)) {
                // println!("eliminating {} for never_exact", s);
                return false;
            }
        }

        for (i, c) in &self.exact {
            if s.as_bytes()[*i] != *c {
                // println!("eliminating {} for exact {:?}", s, (i, *c as char));
                return false;
            }
        }

        for c in &self.any {
            if !matches_any(*c, s) {
                // println!("eliminating {} for any {}", s, *c as char);
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

#[derive(Default)]
struct Analysis {
    possibilities: Vec<Vec<String>>,
}

fn process(target: &str, guesses: &[String]) -> Analysis {
    let mut all_matches: Vec<Matches> = vec![];
    let mut analysis = Analysis::default();
    for guess in guesses {
        let m = Matches::from(&guess, &target);
        all_matches.push(m);
        let possible: Vec<String> = WORDS
            .iter()
            .filter(|w| {
                for m in &all_matches {
                    if !m.matches(w) {
                        return false;
                    }
                }
                true
            })
            .map(|x| x.to_string())
            .collect();
        analysis.possibilities.push(possible);
    }
    analysis
}

use once_cell::sync::Lazy;
use trillium::Conn;
use trillium_router::{Router, RouterConnExt};

static WORDS: Lazy<Vec<&'static str>> = Lazy::new(|| load());

fn boxify(word: &str, target: &str) -> String {
    word.bytes()
        .enumerate()
        .map(|(i, c)| {
            let color = if target.as_bytes()[i] == c {
                "green"
            } else if target.bytes().any(|c2| c == c2) {
                "yellow"
            } else {
                ""
            };
            format!("<div class='letter {}'>{}</div>", color, c as char)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    trillium_smol::run(
        Router::new()
            .get("/", |conn: Conn| async move { conn.ok("hello everyone") })
            .get("/analyze/:guesses", |conn: Conn| async move {
                let guesses: Vec<String> = conn
                    .param("guesses")
                    .unwrap_or("")
                    .split(",")
                    .map(str::to_string)
                    .collect();
                if guesses.len() == 0 {
                    return conn.with_status(400).with_body("provide some words in the url, separated by commas")
                }
                for g in &guesses {
                    if g.len() != 5 {
                        return conn.with_status(400).with_body(format!("invalid guess '{}': must be 5 letters", g));
                    }
                }
                let target: &str = guesses.last().unwrap();
                let analysis = process(target, &guesses);
                let mut response = r#"<html>
<head>
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>
html {
    align-items: center;
    justify-content: center;
    display: flex;
}
body {
    margin-top: 10rem;
    font-family: 'Clear Sans', 'Helvetica Neue', Arial, sans-serif;
}
.poss {
    margin-left: 2rem;
    text-transform: uppercase;
}
.target {
    margin-left: 0.9rem;
}
.word {
    margin-top: 0.5rem;
    display: flex;
}
.letter {
    width: 62px;
    height: 62px;
    font-size: 32px;
    font-weight: 700;
    background-color: #787c7e;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 5px;
    text-transform: uppercase;
}
.letter.green {
    background-color: #6aaa64;
}
.letter.yellow {
    background-color: #c9b458;
}
details {
    margin-left: 1rem;
}
summary::marker {
    color: #787c7e;
}
#signature {
    text-align: right;
}
</style>
</head>
<body>"#
                    .to_string();
                let blank = vec![];
                let len = guesses.len();
                let mut last = WORDS.len() as f64;
                for (i, g) in guesses.iter().enumerate() {
                    let poss = analysis.possibilities.get(i).unwrap_or(&blank);
                    let ratio = last / poss.len() as f64;
                    last = poss.len() as f64;
                    let reduction = if poss.len() > 1 {
                        format!(" ({:.1}x improved)", ratio)
                    } else {
                        String::new()
                    };
                    if i < len - 1 {
                        write!(
                            response,
                            "<div class='word'>{}</div><details><summary>{} words possible{}</summary>\n",
                            boxify(g, target),
                            poss.len(),
                            reduction,
                        )
                        .ok();
                        for p in poss {
                            write!(response, "<div class='poss'>{}</div>\n", p).ok();
                        }
                        write!(response, "</details>\n").ok();
                    } else {
                        write!(response, "<div class='word'>{}</div>\n", boxify(g, target)).ok();
                    }
                }
                write!(response, "<div id=\"signature\"><a href=\"https://wordlyze.crud.net/\">wordlyze.crud.net</a></div>").ok();
                write!(response, "</body>\n").ok();
                conn.with_header("content-type", "text/html").ok(response)
            }),
    );
    Ok(())
}
