use iai;
use std::fs;
use syn_zeug::*;

pub fn count_bases() {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_dna.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    dna.count_elements();
}

pub fn count_bases_simple() {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_dna.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    dna.count_bases();
}

pub fn dna_to_rna() {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_rna.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    dna.convert(SeqKind::Rna);
}

iai::main!(count_bases, count_bases_simple, dna_to_rna);
