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
        let mut chars = dna.chars();
        for i in 0..dna.len() {
            let c = chars.next().unwrap();
            if !(c == 'A' || c == 'C' || c == 'G' || c == 'T') {
                return Err(i as usize);
            }
        }
        return Ok(DNA {
            dna: String::from(dna),
        });
    }

    pub fn into_rna(self) -> RNA {
        let rna = self
            .dna
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => ' ',
            })
            .collect();

        RNA { rna }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut chars = rna.chars();
        for i in 0..rna.len() {
            let c = chars.next().unwrap();
            if !(c == 'A' || c == 'C' || c == 'G' || c == 'U') {
                return Err(i as usize);
            }
        }
        return Ok(RNA {
            rna: String::from(rna),
        });
    }
}
