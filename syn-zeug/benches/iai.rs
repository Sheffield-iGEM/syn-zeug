use std::fs;
use syn_zeug::seq::{Seq, SeqKind};

pub fn rev() {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_dna.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    dna.rev();
}

pub fn count_elements() {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_dna.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    dna.count_elements();
}

pub fn dna_to_rna() {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_rna.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    dna.convert(SeqKind::Rna).unwrap();
}

pub fn reverse_complement_dna() {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_revc.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    dna.reverse_complement().unwrap();
}

iai::main!(rev, count_elements, dna_to_rna, reverse_complement_dna);