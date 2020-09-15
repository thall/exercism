#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        dna.chars()
            .position(|x| valid_dna_nucleotide(x).is_none())
            .map_or(
                Ok(DNA {
                    dna: dna.to_owned(),
                }),
                Err,
            )
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            rna: self.dna.chars().map(|x| convert(x)).collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        rna.chars()
            .position(|x| valid_rna_nucleotide(x).is_none())
            .map_or(
                Ok(RNA {
                    rna: dna.to_owned(),
                }),
                Err,
            )
    }
}

fn valid_dna_nucleotide(c: char) -> Option<char> {
    match c {
        'A' | 'C' | 'G' | 'T' => Some(c),
        _ => None,
    }
}

fn valid_rna_nucleotide(c: char) -> Option<char> {
    match c {
        'A' | 'C' | 'G' | 'U' => Some(c),
        _ => None,
    }
}

fn convert(n: char) -> char {
    match n {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _ => panic!("aa"),
    }
}
