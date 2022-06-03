mod utils;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;
use syn_zeug::seq::{Seq, SeqKind};

fn rev(c: &mut Criterion) {
    bench_time_complexity(c, "rev", "rosalind_dna.txt", Seq::dna, Seq::rev);
}

fn count_elements(c: &mut Criterion) {
    bench_time_complexity(
        c,
        "count_elements",
        "rosalind_dna.txt",
        Seq::dna,
        Seq::count_elements,
    );
}

fn dna_to_rna(c: &mut Criterion) {
    bench_time_complexity(c, "dna_to_rna", "rosalind_dna.txt", Seq::dna, |seq| {
        seq.convert(SeqKind::Rna)
    });
}

fn reverse_complement(c: &mut Criterion) {
    bench_time_complexity(
        c,
        "reverse_complement",
        "rosalind_dna.txt",
        Seq::dna,
        Seq::reverse_complement,
    );
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(1000, Output::Flamegraph(None)));
    targets = rev, count_elements, dna_to_rna, reverse_complement
);
criterion_main!(benches);

fn bench_time_complexity<C, O, R, S, E>(
    c: &mut Criterion,
    bench_name: impl Into<String>,
    data_file: impl AsRef<str>,
    constructor: C,
    routine: R,
) where
    C: Fn(Vec<u8>) -> Result<S, E>,
    R: Fn(&S) -> O,
{
    let data = utils::load_bench_data(data_file);

    let mut group = c.benchmark_group(bench_name);
    for p in 0..=10 {
        let data = data.repeat(2_usize.pow(p));
        let size = data.len() as u64;
        if let Ok(seq) = constructor(data) {
            group.measurement_time(Duration::from_secs(10));
            group.throughput(Throughput::Bytes(size));
            group.bench_with_input(BenchmarkId::from_parameter(size), &seq, |b, seq| {
                b.iter(|| routine(seq));
            });
        }
    }
    group.finish();
}
