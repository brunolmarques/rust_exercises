#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

const RNA: [char; 4] = [ 'C', 'G', 'A', 'U' ];
const DNA: [char; 4] = [ 'G', 'C', 'T', 'A' ];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().enumerate().find(|(_i, c)| !DNA.contains(c)) {
            None => Ok(Dna(String::from(dna))),
            Some((i, _c)) => Err(i),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(
            self.0.chars().map(|c| {
            match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => unreachable!()
            }
        })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().enumerate().find(|(_i, c)| !RNA.contains(c)) {
            None => Ok(Rna(String::from(rna))),
            Some((i, _c)) => Err(i),
        }
    }
}
