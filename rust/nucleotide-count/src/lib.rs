use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> usize {
    dna.chars().filter(|&n| nucleotide == n).count()
}

pub fn nucleotide_counts(dna: &str) -> HashMap<char, usize> {
    let mut count: HashMap<char, usize> = vec![('A', 0), ('T', 0), ('C', 0), ('G', 0)]
                                              .into_iter()
                                              .collect();
    for n in dna.chars() {
        *count.get_mut(&n).unwrap() += 1;
    }
    count
}
