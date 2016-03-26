use std::collections::HashMap;

pub struct Codons {
    pairs: HashMap<String, String>,
    shorthands: HashMap<char, char>,
}

pub fn parse(pairs: Vec<(&str, &str)>) -> Codons {
    let pairs: HashMap<String, String> = pairs.iter()
                                              .map(|&(x, y)| (x.to_owned(), y.to_owned()))
                                              .collect();
    let shorthands = vec![('N', 'T'), ('M', 'C'), ('R', 'A'), ('Y', 'T'), ('H', 'T')]
                         .into_iter()
                         .collect();
    Codons {
        pairs: pairs,
        shorthands: shorthands,
    }
}

impl Codons {
    pub fn name_for(&self, codon: &str) -> Result<&str, &str> {
        let codon = codon.chars()
                         .map(|c| *self.shorthands.get(&c).unwrap_or(&c))
                         .collect::<String>();
        match self.pairs.get(&codon) {
            Some(v) => Ok(v),
            None => Err("not found"),
        }
    }
}
