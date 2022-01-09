use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'C' | 'G' | 'T' | 'A' => {
            let dna_map = nucleotide_counts(dna)?;
            Ok(*dna_map.get(&nucleotide).unwrap_or(&0))
        }
        _ => Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dna_map = HashMap::from_iter( [('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    for c in dna.chars() {
        match c {
            'C' | 'G' | 'T' | 'A' => *dna_map.entry(c).or_default() += 1,
            _ => return Err(c),
        }
    }

    Ok(dna_map)
}