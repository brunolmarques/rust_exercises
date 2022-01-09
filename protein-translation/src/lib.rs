use std::collections::HashMap;
use std::str;

const STOP: &'static str = "stop codon";

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        return self.pairs.get(codon).cloned();
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(str::from_utf8)
            .map(|sequence| self.name_for(sequence.unwrap()))
            .take_while(|&codon| codon != Some(STOP))
            .collect::<Option<Vec<&'a str>>>()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.iter().cloned().collect(),
    }
}
