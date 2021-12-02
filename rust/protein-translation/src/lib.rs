use std::collections::HashMap;
use std::str::from_utf8;

pub struct CodonsInfo<'a> {
    codon_names: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_names.get(&codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(from_utf8)
            .map(|sequence| self.name_for(sequence.unwrap()))
            .take_while(|&codon| codon != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { codon_names: pairs.iter().cloned().collect() }
}
