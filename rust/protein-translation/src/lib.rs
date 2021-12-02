use std::cmp::min;
use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codon_names: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_names.get(&codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut result: Vec<&'a str> = vec![];

        for i in (0..rna.len()).step_by(3) {
            let codon = &rna[i..min(i + 3, rna.len())];
            match self.name_for(codon) {
                None => return None,
                Some(name) => match name {
                    "stop codon" => return Some(result),
                    _ => result.push(name),
                },
            }
        }
        if result.len() > 0 {
            return Some(result);
        }
        None
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codon_names: HashMap<&'a str, &'a str> = HashMap::new();

    for (codon, name) in pairs {
        codon_names.insert(codon, name);
    }

    CodonsInfo { codon_names }
}
