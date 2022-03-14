use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use syn_zeug::*;

pub fn count_bases(c: &mut Criterion) {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_dna.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    c.bench_function("count_elements 250,000", |b| {
        b.iter(|| dna.count_elements())
    });
}

pub fn dna_to_rna(c: &mut Criterion) {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_rna.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    c.bench_function("dna_to_rna 250,000", |b| {
        b.iter(|| dna.convert(SeqKind::Rna))
    });
}

criterion_group!(
    benches, //count_bases,
    dna_to_rna
);
criterion_main!(benches);
