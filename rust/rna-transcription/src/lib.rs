#[derive(PartialEq, Debug)]
pub struct RibonucleicAcid {
    pub strand: String,
}

pub struct DeoxyribonucleicAcid {
    pub strand: String,
}

impl RibonucleicAcid {
    pub fn new(strand: &str) -> RibonucleicAcid {
        RibonucleicAcid { strand: strand.to_owned() }
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(strand: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { strand: strand.to_owned() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid {
            strand: self.strand
                        .chars()
                        .map(|nucleotide| {
                            match nucleotide {
                                'G' => 'C',
                                'C' => 'G',
                                'T' => 'A',
                                'A' => 'U',
                                _ => panic!(),
                            }
                        })
                        .collect(),
        }
    }
}
