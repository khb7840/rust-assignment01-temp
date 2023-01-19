// Solution comes here
pub mod solution {
pub struct Counter {
    pub a: u32,
    pub c: u32,
    pub g: u32,
    pub t: u32,
}

pub fn counts<T: AsRef<[char]>>(input: T) -> Counter {
    let mut counter = Counter { a: 0, c: 0, g: 0, t: 0 };
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
    unimplemented!();
}

pub fn reverse_rna_complement<T: AsRef<[char]>>(input: T) -> Vec<char> {
    unimplemented!();
}

}
