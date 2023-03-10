// Solution comes here

#[allow(dead_code, unused_variables)]

pub struct NucleotideCounter {
    pub a: usize,
    pub c: usize,
    pub g: usize,
    pub t: usize,
}

pub fn counts<T: AsRef<[char]>>(input: T) -> NucleotideCounter {
    let mut counter = NucleotideCounter { a: 0, c: 0, g: 0, t: 0 };
    for c in input.as_ref() {
        match c {
            'A' => counter.a += 1,
            'C' => counter.c += 1,
            'G' => counter.g += 1,
            'T' => counter.t += 1,
            _ => panic!("Invalid nucleotide"),
        }
    }
    counter
}

pub fn dna_complement<T: AsRef<[char]>>(input: T) -> Vec<char> {
    let mut result = Vec::new();
    for c in input.as_ref() {
        match c {
            'A' => result.push('T'),
            'C' => result.push('G'),
            'G' => result.push('C'),
            'T' => result.push('A'),
            _ => panic!("Invalid nucleotide"),
        }
    }
    result
}

// reverse_rna_complement gets DNA as input and returns the reverse complement as RNA
pub fn reverse_rna_complement<T: AsRef<[char]>>(input: T) -> Vec<char> {
    let mut result = Vec::new();
    for c in input.as_ref().iter().rev() {
        match c {
            'A' => result.push('U'),
            'C' => result.push('G'),
            'G' => result.push('C'),
            'T' => result.push('A'),
            _ => panic!("Invalid nucleotide"),
        }
    }
    result
}

