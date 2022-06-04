mod utils;

use once_cell::sync::Lazy;
use syn_zeug::seq::{Kind, Seq};

// TODO: Give this a second look...
static AMBIGUOUS: Lazy<Vec<u8>> = Lazy::new(|| [utils::load_bench_data("ambiguous_seq.txt"), b"X".to_vec()].concat());
static DNA: Lazy<Seq> = Lazy::new(|| Seq::dna(utils::load_bench_data("rosalind_dna.txt")).unwrap());

// TODO: Do a need to add black_boxes?

fn new() {
    dbg!(Seq::new(&*AMBIGUOUS).unwrap());
}

fn rev() {
    DNA.rev();
}

fn count_elements() {
    DNA.count_elements();
}

fn dna_to_rna() {
    DNA.convert(Kind::Rna).unwrap();
}

fn reverse_complement_dna() {
    DNA.reverse_complement().unwrap();
}

iai::main!(new, rev, count_elements, dna_to_rna, reverse_complement_dna);
