use std::collections::HashSet;
use std::ascii::AsciiExt;

pub fn is_pangram(text: &str) -> bool {
    let letters: HashSet<char> = text.chars()
                                     .filter(|c| c.is_alphabetic() && c.is_ascii())
                                     .map(|c| c.to_lowercase().next().unwrap())
                                     .collect();
    letters.len() == 26
}
