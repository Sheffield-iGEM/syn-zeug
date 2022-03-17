use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
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
    c.bench_function("count_elements 1kb", |b| b.iter(|| dna.count_elements()));
}

pub fn dna_to_rna(c: &mut Criterion) {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_rna.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    c.bench_function("dna_to_rna 1kb", |b| b.iter(|| dna.convert(SeqKind::Rna)));
}

pub fn reverse_complement_dna(c: &mut Criterion) {
    let dna = Seq::dna(
        fs::read_to_string("benches/data/rosalind_revc.txt")
            .unwrap()
            .trim()
            .as_bytes(),
    )
    .unwrap();
    c.bench_function("reverse_complement_dna 1kb", |b| {
        b.iter(|| dna.reverse_complement())
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(1000, Output::Flamegraph(None)));
    targets = count_bases, dna_to_rna, reverse_complement_dna
);
criterion_main!(benches);
