
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

fn write_words(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let words = {
        let mut fin = File::open(src)?;
        let mut words: String = String::new();
        fin.read_to_string(&mut words)?;
        words
    };

    let mut out = env::var("OUT_DIR")?;
    out.push_str(dest);
    let mut fout = File::create(out)?;

    write!(fout, "[")?;
    for word in words.lines() {
        writeln!(fout, "\"{word}\",")?;
    }
    write!(fout, "]")?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    write_words("src/wordle-nyt-answers-alphabetical.txt", "/answers.in.rs")?;
    write_words("src/wordle-nyt-allowed-guesses.txt", "/guesses.in.rs")?;
    Ok(())
}
