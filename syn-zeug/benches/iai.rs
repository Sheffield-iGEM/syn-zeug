mod utils;

use once_cell::sync::Lazy;
use syn_zeug::seq::{Seq, SeqKind};

static DNA: Lazy<Seq> = Lazy::new(|| Seq::dna(utils::load_bench_data("rosalind_dna.txt")).unwrap());

fn rev() {
    DNA.rev();
}

fn count_elements() {
    DNA.count_elements();
}

fn dna_to_rna() {
    DNA.convert(SeqKind::Rna).unwrap();
}

fn reverse_complement_dna() {
    DNA.reverse_complement().unwrap();
}

iai::main!(rev, count_elements, dna_to_rna, reverse_complement_dna);
