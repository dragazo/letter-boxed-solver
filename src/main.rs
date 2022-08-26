use std::collections::{BTreeMap, BTreeSet};

use clap::Parser;

lazy_static::lazy_static! {
    static ref WORD_LIST: BTreeSet<String> = {
        include_str!("word-list.txt").lines().map(|x| x.trim().to_ascii_lowercase()).filter(|x| !x.is_empty()).collect()
    };
}

fn solve_recursive<'a>(words: &mut Vec<&'a str>, remaining_letters: &BTreeSet<char>, max_words: usize, legal_words: &BTreeSet<&'a str>) -> bool {
    assert!(words.len() <= max_words);
    if remaining_letters.is_empty() { return true }
    if words.len() == max_words { return false }

    fn rank_words<'a, I>(words: I, remaining_letters: &BTreeSet<char>) -> BTreeSet<(usize, &'a str)> where I: Iterator<Item = &'a str> {
        words.map(|x| (remaining_letters.len() - x.chars().filter(|ch| remaining_letters.contains(&ch)).collect::<BTreeSet<_>>().len(), x)).collect()
    }
    let ranked_words = match words.last() {
        Some(last) => {
            let ch = last.chars().rev().next().unwrap();
            rank_words(legal_words.iter().copied().filter(|x| x.chars().next().unwrap() == ch), remaining_letters)
        }
        None => rank_words(legal_words.iter().copied(), remaining_letters),
    };

    for (_, choice) in ranked_words.iter().copied() {
        words.push(choice);
        let new_remaining_letters = remaining_letters - &choice.chars().collect::<BTreeSet<_>>();
        if solve_recursive(words, &new_remaining_letters, max_words, legal_words) { return true }
        words.pop();
    }

    false
}

#[derive(Parser)]
struct Args {
    letters: String,
    groups: usize,
}

fn main() {
    macro_rules! crash {
        ($code:literal : $($msg:tt)*) => {
            eprintln!($($msg)*);
            std::process::exit($code);
        }
    }

    let mut args = Args::parse();
    args.letters = args.letters.to_ascii_lowercase();

    if args.letters.is_empty() { crash!(1: "letter list cannot be empty"); }
    if args.groups == 0 { crash!(1: "group size cannot be zero"); }
    if args.letters.chars().count() % args.groups != 0 { crash!(1: "letters list must be divisible by group size"); }
    if args.letters.chars().collect::<BTreeSet<_>>().len() != args.letters.chars().count() { crash!(1: "repeated letters are not allowed"); }

    let letter_to_bucket: BTreeMap<_,_> = args.letters.chars().enumerate().map(|(i, ch)| (ch, i / args.groups)).collect();

    let legal_words: BTreeSet<&str> = WORD_LIST.iter().map(String::as_str).filter(|word| {
        let mut prev_bucket = match letter_to_bucket.get(&word.chars().next().unwrap()) {
            Some(x) => *x,
            None => return false,
        };
        for ch in word.chars().skip(1) {
            let bucket = match letter_to_bucket.get(&ch) {
                Some(x) => *x,
                None => return false,
            };
            if bucket == prev_bucket { return false }
            prev_bucket = bucket;
        }
        true
    }).collect();

    for max_words in 1.. {
        let mut solution = Vec::with_capacity(max_words);
        let remaining_letters = args.letters.chars().collect();
        if solve_recursive(&mut solution, &remaining_letters, max_words, &legal_words) {
            println!("solution: {solution:?}");
            break;
        }
    }
}
