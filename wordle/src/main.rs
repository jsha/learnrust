use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("/usr/share/dict/words")?;
    let f = BufReader::new(f);
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
            for i in 0..5 {
                counts[i][line.as_bytes()[i] as usize] += 1;
            }
        }
    }
    for i in 0..26 {
        let letter = 'a' as u8 + i;
        print!("{}: ", letter as char);
        for j in 0..5 {
            print!("{}\t", counts[j][letter as usize]);
        }
        println!();
    }

    let f = File::open("/usr/share/dict/words")?;
    let f = BufReader::new(f);
    for line in f.lines() {
        let line = line?;
        if line.len() == 5 && line.is_ascii() {
            let mut score = 1;
            for i in 0..5 {
                score *= counts[i][line.as_bytes()[i] as usize]
            }
            println!("{}: {}", line, score)
        }
    }
    Ok(())
}
