use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(nucleotide) {
        return Result::Err(nucleotide)
    }
    dna.chars()
        .try_fold(0,
            |acc: usize, c| if is_valid(c) {
                Result::Ok(acc + if c == nucleotide {1} else {0})
            } else {
                Result::Err(c)
            }
        )
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    let chars = vec!['A', 'G', 'T', 'C'];
    for ch in chars {
        let count = count(ch, dna);
        println!("{:?}", count);
        if count.ok().is_some() {
            counts.insert(ch, count.unwrap());
        } else {
            return Result::Err(count.unwrap_err());
        }
    }
    Result::Ok(counts)
}

pub fn is_valid(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'G' | 'T' | 'C' => true,
        _ => false,
    }
}
