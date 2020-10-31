use std::collections::HashMap;

fn valid_nucleotide(c: char) -> bool {
    c == 'A' || c == 'C' || c == 'G' || c == 'T'
}

fn valid_dna(dna: &str) -> Result<bool, char> {
    for c in dna.chars() {
        if !(valid_nucleotide(c)) {
            return Err(c);
        }
    }
    return Ok(true);
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match valid_nucleotide(nucleotide) {
        true => match valid_dna(dna) {
            Ok(_) => Ok(dna.matches(nucleotide).count()),
            Err(c) => Err(c),
        },
        false => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    counts.insert('A', count('A', dna)?);
    counts.insert('C', count('C', dna)?);
    counts.insert('G', count('G', dna)?);
    counts.insert('T', count('T', dna)?);

    Ok(counts)
}
