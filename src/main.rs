use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use console::Term;
use rand::seq::SliceRandom;
use structopt::StructOpt;
use itertools::Itertools;

const WHITELISTED_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, StructOpt)]
#[structopt(name = "Word Guesser", about = "Simple word-guessing game.")]
struct Opt {
    /// Path to wordlist
    #[structopt(short = "w", long = "wordlist")]
    wordlist_path: PathBuf,

    /// Character hidden characters show as
    #[structopt(short, long, default_value = ".")]
    character: char,
}

fn read_wordlist(path: PathBuf) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let term = Term::stdout();

    let wordlist = read_wordlist(opt.wordlist_path)?;

    let word = wordlist.choose(&mut rand::thread_rng()).unwrap();
    let mut guessed_chars: Vec<char> = vec![];
    let mut guesses = 0;

    let fill_character = opt.character.clone();

    loop {
        term.clear_screen()?;

        let revealed_word = word.to_lowercase().chars().map(|c| {
            if !WHITELISTED_CHARS.contains(c) { return c };

            match guessed_chars.contains(&c) {
                true => c,
                false => fill_character,
            }
        }).collect::<String>();


        term.write_line(&format!("{}\n", revealed_word))?;

        term.write_line(&format!(
            "Guessed: {}",
            guessed_chars
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ))?;
        term.write_line(&format!("Guesses: {}", guesses))?;

        if revealed_word == word.to_lowercase() {
            term.write_line(&format!("You win! The word was {} and it took you {} tries to guess it.", word, guesses))?;
            let unique_chars_in_word = word.chars().sorted().dedup().count();
            term.write_line(&format!("Optimal amount of guesses: {}", unique_chars_in_word))?;
            std::process::exit(0);
        }

        let c = term.read_char()?.to_ascii_lowercase();
        if !guessed_chars.contains(&c) {
            guessed_chars.push(c);
            guesses += 1;
        }
    }
}
