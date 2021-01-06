use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();
    for factor in factors {
        if *factor == 0 {
            continue;
        }

        let mut current = *factor;
        while current < limit {
            multiples.insert(current);
            current += factor;
        }
    }

    multiples.iter().sum()
}
