use std::collections::HashMap;

pub fn hex_to_int(n: &str) -> Option<u32> {
    let hex_digits = "0123456789abcdef";
    let hex_digits: HashMap<char, u32> = hex_digits.chars()
                                                   .enumerate()
                                                   .map(|(i, c)| (c, i as u32))
                                                   .collect();
    if n.chars().all(|c| hex_digits.contains_key(&c)) {
        Some(n.chars().rev().enumerate().fold(0, |acc, (i, n)| {
            16u32.pow(i as u32) * hex_digits.get(&n).unwrap() + acc
        }))
    } else {
        None
    }
}
