use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    let mut count = HashMap::<String, u32>::new();
    let text: String = text.chars()
                           .map(|c| {
                               if c.is_alphanumeric() {
                                   c.to_lowercase().next().unwrap()
                               } else {
                                   ' '
                               }
                           })
                           .collect();
    for word in text.split_whitespace() {
        *count.entry(word.to_owned()).or_insert(0) += 1;
    }
    count
}
