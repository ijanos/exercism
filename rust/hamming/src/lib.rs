pub fn hamming_distance(text1: &str, text2: &str) -> Result<usize, &'static str> {
    if text1.len() == text2.len() {
        Ok(text1.chars().zip(text2.chars()).filter(|&(c1, c2)| c1 != c2).count())
    } else {
        Err("inputs of different length")
    }
}
