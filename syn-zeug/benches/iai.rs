mod utils;

use syn_zeug::seq::{Seq, SeqKind};

pub fn rev() {
    let dna = Seq::dna(utils::load_bench_data("rosalind_dna.txt")).unwrap();
    dna.rev();
}

pub fn count_elements() {
    let dna = Seq::dna(utils::load_bench_data("rosalind_dna.txt")).unwrap();
    dna.count_elements();
}

pub fn dna_to_rna() {
    let dna = Seq::dna(utils::load_bench_data("rosalind_rna.txt")).unwrap();
    dna.convert(SeqKind::Rna).unwrap();
}

pub fn reverse_complement_dna() {
    let dna = Seq::dna(utils::load_bench_data("rosalind_revc.txt")).unwrap();
    dna.reverse_complement().unwrap();
}

iai::main!(rev, count_elements, dna_to_rna, reverse_complement_dna);
