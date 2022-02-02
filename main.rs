use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_words(filename: &String) -> Vec<String> {
    let mut words = Vec::new();
    for word in read_lines(filename).unwrap() {
        words.push(word.unwrap());
    }
    return words;
}

fn filter_guess_for_target(guess: &String, target: &String, words: &mut Vec<&String>) {
    // for each character evaluate the information we get for the given guess with the simulated target
    for (idx, chr) in guess.chars().enumerate() {
        // filter down the word list based on the information gained
        if target.chars().nth(idx).unwrap() == chr {
            words.retain(|possible_elim| possible_elim.chars().nth(idx).unwrap() != chr);
        } else if target.contains(chr) {
            words.retain(|possible_elim| possible_elim.contains(chr));
        }
    }
}

fn score_guess(guesses: &Vec<&String>, words: &Vec<String>) -> usize {
    // run a scenario for each word, imagining that it is the target
    let mut eliminates = 0;

    // pick a word that could be the actual target for the guess we are evaluating
    for possible_target in words {
        let mut remaining_words: Vec<&String> = words.iter().collect();
        for guess in guesses {
            filter_guess_for_target(guess, possible_target, &mut remaining_words);
        }
        eliminates += words.len() - remaining_words.len();
    }

    // println!("evaluated guesses: {:?}, score: {}", guesses, eliminates);

    return eliminates;
}

fn main() {
    let guesses = load_words(&String::from("./wordle-guesses.txt")); // does not include answers.
    let answers = load_words(&String::from("./wordle-answers.txt"));
    let mut base_guess_vec: Vec<&String> = Vec::new();

    let mut output = BufWriter::new(File::create("output.txt").unwrap());

    for i in 0..5 {
        let mut rankings: Vec<(usize, &String)> = guesses
            .par_iter()
            .progress_count(guesses.len() as u64)
            .map(|word| {
                let mut guess_vec: Vec<&String> = base_guess_vec.clone();
                guess_vec.push(&word);
                return (score_guess(&guess_vec, &answers), word);
            })
            .collect();
        rankings.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let (best_score, best_word) = &rankings.first().unwrap();
        base_guess_vec.push(best_word);
        println!("guess {}", i);
        write!(&mut output, "guess {}\n", i);
        for (score, word) in rankings.drain(0..100) {
            println!("{:?},{:?},{:?}", base_guess_vec, score, word);
            write!(&mut output, "{:?},{:?},{:?}\n", base_guess_vec, score, word);
        }
    }
}
