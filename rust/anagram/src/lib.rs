use std::collections::HashMap;

fn count_chars(word: &str) -> HashMap<char, u32> {
    let mut charcount = HashMap::<char, u32>::new();
    for c in word.chars() {
        let c = c.to_lowercase().next().unwrap();
        *charcount.entry(c).or_insert(0) += 1;
    }
    charcount
}

pub fn anagrams_for(word: &str, inputs: &[&str]) -> Vec<String> {
    let charcount = count_chars(word);
    inputs.iter()
          .filter(|&input| word.to_lowercase() != input.to_lowercase())
          .filter(|&input| count_chars(input) == charcount)
          .map(|&w| w.to_owned())
          .collect::<Vec<String>>()
}
